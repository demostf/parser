use fnv::FnvHashMap;
use std::borrow::Cow;
use std::collections::HashMap;

use crate::demo::gamevent::GameEventDefinition;

use crate::demo::message::packetentities::{
    EntityId, PacketEntitiesMessage, PacketEntity, UpdateType,
};
use crate::demo::message::stringtable::StringTableMeta;
use crate::demo::message::{Message, MessageType};
use crate::demo::packet::datatable::{
    ClassId, ParseSendTable, SendTable, SendTableName, ServerClass,
};
use crate::demo::packet::stringtable::StringTableEntry;

use crate::demo::data::DemoTick;
use crate::demo::sendprop::{SendProp, SendPropIdentifier};
use crate::nullhasher::NullHasherBuilder;
use crate::{Result, Stream};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
#[cfg(feature = "trace")]
use tracing::warn;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct DemoMeta {
    pub version: u16,
    pub game: String,
    pub interval_per_tick: f32,
}

#[derive(Clone)]
pub struct ParserState {
    pub static_baselines: HashMap<ClassId, StaticBaseline, NullHasherBuilder>,
    pub parsed_static_baselines: RefCell<HashMap<ClassId, Vec<SendProp>, NullHasherBuilder>>,
    pub event_definitions: Vec<GameEventDefinition>,
    pub string_tables: Vec<StringTableMeta>,
    pub entity_classes: HashMap<EntityId, ClassId, NullHasherBuilder>,
    // indexed by ClassId
    pub send_tables: Vec<SendTable>,
    pub server_classes: Vec<ServerClass>,
    pub instance_baselines: [Baseline; 2],
    pub demo_meta: DemoMeta,
    analyser_handles: fn(message_type: MessageType) -> bool,
    handle_entities: bool,
    parse_all: bool,
    pub protocol_version: u32,
}

#[derive(Clone)]
pub struct StaticBaseline {
    pub class_id: ClassId,
    pub raw: Stream<'static>,
}

impl StaticBaseline {
    fn new(class_id: ClassId, raw: Stream<'static>) -> Self {
        StaticBaseline { class_id, raw }
    }

    pub fn parse(&self, send_table: &SendTable) -> Result<Vec<SendProp>> {
        let mut props = Vec::with_capacity(8);
        PacketEntitiesMessage::read_update(
            &mut self.raw.clone(),
            send_table,
            &mut props,
            0u32.into(),
        )?;
        Ok(props)
    }
}

impl<'a> ParserState {
    pub fn new(
        protocol_version: u32,
        analyser_handles: fn(message_type: MessageType) -> bool,
        parse_all: bool,
    ) -> Self {
        ParserState {
            static_baselines: HashMap::with_hasher(NullHasherBuilder),
            parsed_static_baselines: RefCell::new(HashMap::with_hasher(NullHasherBuilder)),
            event_definitions: Vec::new(),
            string_tables: Vec::new(),
            entity_classes: HashMap::with_hasher(NullHasherBuilder),
            send_tables: Vec::new(),
            server_classes: Vec::new(),
            instance_baselines: [Baseline::default(), Baseline::default()],
            demo_meta: DemoMeta::default(),
            analyser_handles,
            handle_entities: analyser_handles(MessageType::PacketEntities) || parse_all,
            parse_all,
            protocol_version,
        }
    }

    pub fn get_static_baseline(
        &self,
        class_id: ClassId,
        send_table: &SendTable,
    ) -> Result<Vec<SendProp>> {
        match self.static_baselines.get(&class_id) {
            Some(static_baseline) => static_baseline.parse(send_table),
            None => {
                #[cfg(feature = "trace")]
                warn!(
                    class_id = display(class_id),
                    "class without static baseline"
                );
                Ok(Vec::new())
            }
        }
        // let mut cached = self.parsed_static_baselines.borrow_mut();
        // Ok(match cached.entry(class_id) {
        //     Entry::Occupied(entry) => entry.get().as_slice(),
        //     Entry::Vacant(entry) => match self.static_baselines.get(&class_id) {
        //         Some(static_baseline) => {
        //             let props = static_baseline.parse(send_table)?;
        //             entry.insert(props).as_slice()
        //         }
        //         None => {
        //             #[cfg(feature = "trace")]
        //             warn!(
        //                 class_id = display(class_id),
        //                 "class without static baseline"
        //             );
        //             &[]
        //         }
        //     },
        // })
    }

    pub fn get_baseline(
        &self,
        baseline_index: usize,
        entity_index: EntityId,
        class_id: ClassId,
        send_table: &SendTable,
        is_delta: bool,
    ) -> Result<Cow<[SendProp]>> {
        match self.instance_baselines[baseline_index].get(entity_index) {
            Some(baseline) if baseline.server_class == class_id && is_delta => {
                Ok(Cow::Borrowed(&baseline.props))
            }
            _ => match self.static_baselines.get(&class_id) {
                Some(_static_baseline) => {
                    Ok(Cow::Owned(self.get_static_baseline(class_id, send_table)?))
                }
                None => {
                    #[cfg(feature = "trace")]
                    warn!(
                        class_id = display(class_id),
                        "class without static baseline"
                    );
                    Ok(Cow::Owned(Vec::new()))
                }
            },
        }
    }

    pub fn handle_data_table(
        &mut self,
        parse_tables: Vec<ParseSendTable>,
        server_classes: Vec<ServerClass>,
    ) -> Result<()> {
        if self.handle_entities {
            let mut send_tables: FnvHashMap<SendTableName, SendTable> = parse_tables
                .iter()
                .map(|parse_table| {
                    let flat = parse_table.flatten_props(&parse_tables);
                    Ok((
                        parse_table.name.clone(),
                        SendTable {
                            name: parse_table.name.clone(),
                            needs_decoder: parse_table.needs_decoder,
                            raw_props: parse_table.props.clone(),
                            flattened_props: flat?,
                        },
                    ))
                })
                .collect::<Result<_>>()?;

            self.server_classes = server_classes;

            self.send_tables.reserve(self.server_classes.len());

            for class in self.server_classes.iter() {
                if let Some(table) = send_tables.remove(&class.data_table) {
                    self.send_tables.push(table);
                } else {
                    #[cfg(feature = "trace")]
                    warn!(class = debug(class), "class without table");
                }
            }
        }

        Ok(())
    }

    pub fn handle_string_table_meta(&mut self, table: StringTableMeta) {
        self.string_tables.push(table);
    }

    pub fn should_parse_message(&self, message_type: MessageType) -> bool {
        self.parse_all
            || if message_type == MessageType::PacketEntities {
                self.handle_entities
            } else {
                Self::does_handle(message_type) || (self.analyser_handles)(message_type)
            }
    }

    pub fn does_handle(message_type: MessageType) -> bool {
        matches!(
            message_type,
            MessageType::ServerInfo
                | MessageType::NetTick
                | MessageType::GameEventList
                | MessageType::CreateStringTable
                | MessageType::PacketEntities
                | MessageType::UpdateStringTable
        )
    }

    pub fn handle_message(&mut self, message: Message, _tick: DemoTick) {
        match message {
            Message::ServerInfo(message) => {
                self.demo_meta.version = message.version;
                self.demo_meta.game = message.game;
                self.demo_meta.interval_per_tick = message.interval_per_tick;
            }
            Message::GameEventList(message) => {
                self.event_definitions = message.event_list;
            }
            Message::PacketEntities(ent_message) => {
                for removed in ent_message.removed_entities.iter() {
                    self.entity_classes.remove(removed);
                }

                for entity in ent_message.entities.iter() {
                    if entity.update_type == UpdateType::Delete {
                        self.entity_classes.remove(&entity.entity_index);
                    }
                    self.entity_classes
                        .insert(entity.entity_index, entity.server_class);
                }

                if ent_message.updated_base_line {
                    let old_index = ent_message.base_line as usize;
                    let new_index = 1 - old_index;
                    let [baseline1, baseline2] = &mut self.instance_baselines;
                    if old_index == 0 {
                        baseline2.copy_from(baseline1);
                    } else {
                        baseline1.copy_from(baseline2);
                    }

                    for entity in ent_message.entities {
                        if entity.update_type == UpdateType::Enter {
                            let updated_baseline =
                                match self.instance_baselines[old_index].get(entity.entity_index) {
                                    Some(baseline_entity)
                                        if baseline_entity.server_class == entity.server_class
                                            && ent_message.delta.is_some() =>
                                    {
                                        let mut updated_baseline = baseline_entity.clone();
                                        updated_baseline.apply_update(&entity.props);
                                        updated_baseline
                                    }
                                    _ => entity.into(),
                                };
                            self.instance_baselines[new_index].set(updated_baseline);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn handle_string_entry(
        &mut self,
        table: &str,
        _index: usize,
        entry: &StringTableEntry<'a>,
    ) {
        if table == "instancebaseline" {
            if let (Some(extra), Ok(class_id)) = (&entry.extra_data, entry.text().parse()) {
                let baseline = StaticBaseline::new(class_id, extra.data.to_owned());
                self.static_baselines.insert(class_id, baseline);
                self.parsed_static_baselines.borrow_mut().remove(&class_id);
            }
        }
    }

    pub fn index_for_prop(&self, class: ClassId, prop: SendPropIdentifier) -> Option<u32> {
        let send_table = self.send_tables.get(usize::from(class))?;
        send_table
            .flattened_props
            .iter()
            .enumerate()
            .find(|(_i, def)| def.identifier == prop)
            .map(|(index, _)| index as u32)
    }
}

#[derive(Clone)]
pub struct Baseline {
    instances: Vec<Option<BaselineEntity>>,
}

impl Default for Baseline {
    fn default() -> Self {
        Baseline {
            instances: vec![None; 2048],
        }
    }
}

impl Baseline {
    pub fn get(&self, index: EntityId) -> Option<&BaselineEntity> {
        self.instances
            .get(usize::from(index))
            .and_then(|opt| opt.as_ref())
    }

    fn set(&mut self, entity: BaselineEntity) {
        let index = entity.entity_id;
        self.instances[usize::from(index)] = Some(entity);
    }

    pub fn keys<'a>(&'a self) -> impl Iterator<Item = EntityId> + 'a {
        self.instances
            .iter()
            .filter_map(|entity| entity.as_ref().map(|entity| entity.entity_id))
    }

    pub fn into_values(self) -> impl Iterator<Item = PacketEntity> {
        self.instances
            .into_iter()
            .filter_map(|entity| entity.map(|entity| entity.into()))
    }

    pub fn contains(&self, index: EntityId) -> bool {
        self.get(index).is_some()
    }

    fn copy_from(&mut self, other: &Baseline) {
        for (ent, other_ent) in self.instances.iter_mut().zip(other.instances.iter()) {
            match (ent, other_ent) {
                (ent, Some(other_ent)) => *ent = Some(other_ent.clone()),
                (ent, None) => {
                    *ent = None;
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct BaselineEntity {
    pub entity_id: EntityId,
    pub server_class: ClassId,
    pub props: Vec<SendProp>,
    pub serial: u32,
}

impl BaselineEntity {
    fn mut_prop_by_identifier(&mut self, index: &SendPropIdentifier) -> Option<&mut SendProp> {
        self.props.iter_mut().find(|prop| prop.identifier == *index)
    }

    pub fn apply_update(&mut self, props: &[SendProp]) {
        for prop in props {
            match self.mut_prop_by_identifier(&prop.identifier) {
                Some(existing_prop) => existing_prop.value = prop.value.clone(),
                None => self.props.push(prop.clone()),
            }
        }
    }
}

impl From<PacketEntity> for BaselineEntity {
    fn from(entity: PacketEntity) -> Self {
        BaselineEntity {
            entity_id: entity.entity_index,
            server_class: entity.server_class,
            props: entity.props,
            serial: entity.serial_number,
        }
    }
}

impl From<BaselineEntity> for PacketEntity {
    fn from(baseline: BaselineEntity) -> Self {
        PacketEntity {
            server_class: baseline.server_class,
            entity_index: baseline.entity_id,
            props: baseline.props,
            in_pvs: false,
            update_type: UpdateType::Enter,
            serial_number: baseline.serial,
            delay: None,
            delta: None,
            baseline_index: 0,
        }
    }
}
