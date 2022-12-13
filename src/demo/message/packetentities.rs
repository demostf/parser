use bitbuffer::{BitRead, BitReadSized, BitWrite, BitWriteSized, BitWriteStream, LittleEndian};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::borrow::Cow;

use crate::demo::message::stringtable::log_base2;
use crate::demo::packet::datatable::{ClassId, SendTable};
use crate::demo::parser::{Encode, ParseBitSkip};
use crate::demo::sendprop::{SendProp, SendPropIdentifier, SendPropValue};
use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};
use parse_display::{Display, FromStr};
use std::cmp::{min, Ordering};
use std::collections::HashSet;

use crate::demo::data::ServerTick;
use itertools::Either;
use std::fmt;
#[cfg(feature = "trace")]
use tracing::trace;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Display,
    Ord,
    PartialOrd,
    FromStr,
    Default,
)]
pub struct EntityId(u32);

impl From<u32> for EntityId {
    fn from(num: u32) -> Self {
        EntityId(num)
    }
}

impl From<EntityId> for u32 {
    fn from(id: EntityId) -> Self {
        id.0
    }
}

impl From<usize> for EntityId {
    fn from(num: usize) -> Self {
        EntityId(num as u32)
    }
}

impl From<EntityId> for usize {
    fn from(id: EntityId) -> Self {
        id.0 as usize
    }
}

impl PartialEq<u32> for EntityId {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u32> for EntityId {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    BitRead, BitWrite, Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr,
)]
#[discriminant_bits = 2]
#[repr(u8)]
pub enum UpdateType {
    Preserve = 0b00,
    Leave = 0b01,
    Enter = 0b10,
    Delete = 0b11,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacketEntity {
    pub server_class: ClassId,
    pub entity_index: EntityId,
    pub props: Vec<SendProp>,
    pub in_pvs: bool,
    pub update_type: UpdateType,
    pub serial_number: u32,
    pub delay: Option<f32>,
    pub delta: Option<ServerTick>,
    pub baseline_index: usize,
}

impl fmt::Display for PacketEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}({}) {{", self.entity_index, self.server_class)?;
        for child in self.props.iter() {
            writeln!(f, "\t{}", child)?;
        }
        write!(f, "}}")
    }
}

impl PacketEntity {
    fn mut_prop_by_identifier(&mut self, index: &SendPropIdentifier) -> Option<&mut SendProp> {
        self.props.iter_mut().find(|prop| prop.identifier == *index)
    }

    pub fn get_prop_by_identifier(
        &self,
        index: &SendPropIdentifier,
        parser_state: &ParserState,
    ) -> Option<SendProp> {
        self.props(parser_state)
            .find(|prop| prop.identifier == *index)
    }

    pub fn apply_update(&mut self, props: &[SendProp]) {
        for prop in props {
            match self.mut_prop_by_identifier(&prop.identifier) {
                Some(existing_prop) => existing_prop.value = prop.value.clone(),
                None => self.props.push(prop.clone()),
            }
        }
    }

    pub fn get_prop_by_name(
        &self,
        table_name: &str,
        name: &str,
        parser_state: &ParserState,
    ) -> Option<SendProp> {
        let identifier = SendPropIdentifier::new(table_name, name);
        self.get_prop_by_identifier(&identifier, parser_state)
    }

    pub fn get_baseline_props<'a>(&self, parser_state: &'a ParserState) -> Cow<'a, [SendProp]> {
        parser_state
            .get_baseline(
                self.baseline_index,
                self.entity_index,
                self.server_class,
                &parser_state.send_tables[usize::from(self.server_class)],
                self.delta.is_some(),
            )
            .unwrap_or_default()
    }

    pub fn props<'a>(
        &'a self,
        parser_state: &'a ParserState,
    ) -> impl Iterator<Item = SendProp> + 'a {
        if self.update_type == UpdateType::Enter {
            let mut found_props = HashSet::<SendPropIdentifier>::new();
            let props = self.props.iter().cloned();
            let baseline_props = self
                .get_baseline_props(parser_state)
                .into_owned()
                .into_iter();
            Either::Left(props.chain(baseline_props).filter(move |prop| {
                let found = found_props.contains(&prop.identifier);
                found_props.insert(prop.identifier);
                !found
            }))
        } else {
            Either::Right(self.props.iter().cloned())
        }
    }
}

fn read_bit_var<'a, T: BitReadSized<'a, LittleEndian>>(stream: &mut Stream<'a>) -> ReadResult<T> {
    let ty: u8 = stream.read_sized(2)?;

    let bits = match ty {
        0 => 4,
        1 => 8,
        2 => 12,
        _ => 32,
    };
    stream.read_sized(bits)
}

fn write_bit_var(var: u32, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
    let (ty, bits): (u8, usize) = if var >= 2u32.pow(12) {
        (3, 32)
    } else if var >= 2u32.pow(8) {
        (2, 12)
    } else if var >= 2u32.pow(4) {
        (1, 8)
    } else {
        (0, 4)
    };

    ty.write_sized(stream, 2)?;
    var.write_sized(stream, bits)
}

#[test]
fn test_bit_var_roundtrip() {
    use bitbuffer::{BitReadBuffer, BitReadStream};

    fn bit_var_normal(val: u32) {
        let mut data = Vec::with_capacity(16);
        let pos = {
            let mut write = BitWriteStream::new(&mut data, LittleEndian);
            write_bit_var(val, &mut write).unwrap();
            write.bit_len()
        };
        let mut read = BitReadStream::new(BitReadBuffer::new(&data, LittleEndian));
        assert_eq!(val, read_bit_var::<u32>(&mut read).unwrap());
        assert_eq!(pos, read.pos());
    }
    bit_var_normal(0);
    bit_var_normal(1);
    bit_var_normal(24);
    bit_var_normal(1234);
    bit_var_normal(12345);
    bit_var_normal(123456);
    bit_var_normal(1234567);
    bit_var_normal(12345678);
    bit_var_normal(123456789);
    bit_var_normal(u32::MAX);

    for i in 0..31 {
        bit_var_normal(2u32.pow(i) as u32);
        bit_var_normal(2u32.pow(i) as u32 - 1);
        bit_var_normal(2u32.pow(i) as u32 + 1);
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct PacketEntitiesMessage {
    pub entities: Vec<PacketEntity>,
    pub removed_entities: Vec<EntityId>,
    pub max_entries: u16,
    pub delta: Option<ServerTick>,
    pub base_line: u8,
    pub updated_base_line: bool,
}

fn get_send_table(state: &ParserState, class: ClassId) -> Result<&SendTable> {
    state
        .send_tables
        .get(usize::from(class))
        .ok_or(ParseError::UnknownServerClass(class))
}

fn get_entity_for_update(
    state: &ParserState,
    entity_index: EntityId,
    update_type: UpdateType,
    delta: Option<ServerTick>,
) -> Result<PacketEntity> {
    let class_id = *state
        .entity_classes
        .get(&entity_index)
        .ok_or(ParseError::UnknownEntity(entity_index))?;

    Ok(PacketEntity {
        server_class: class_id,
        entity_index,
        props: Vec::with_capacity(8),
        in_pvs: false,
        update_type,
        serial_number: 0,
        delay: None,
        delta,
        baseline_index: 0,
    })
}

impl Parse<'_> for PacketEntitiesMessage {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let max_entries = stream.read_sized(11)?;
        let delta: Option<ServerTick> = stream.read()?;
        let base_line = stream.read_sized(1)?;
        let updated_entries: u16 = stream.read_sized(11)?;
        let length: u32 = stream.read_sized(20)?;
        let updated_base_line = stream.read()?;

        let mut data = stream.read_bits(length as usize)?;

        let mut entities = Vec::with_capacity(min(updated_entries, 128) as usize);
        let mut removed_entities = Vec::new();

        let mut last_index: i32 = -1;

        for _ in 0..updated_entries {
            let diff: u32 = read_bit_var(&mut data)?;
            last_index = last_index.saturating_add(diff as i32).saturating_add(1);
            if last_index >= 2048 {
                return Err(ParseError::InvalidDemo("invalid entity index"));
            }
            let entity_index = EntityId::from(last_index as u32);

            let update_type = data.read()?;
            if update_type == UpdateType::Enter {
                let mut entity =
                    Self::read_enter(&mut data, entity_index, state, base_line as usize, delta)?;
                let send_table = get_send_table(state, entity.server_class)?;
                Self::read_update(&mut data, send_table, &mut entity.props, entity_index)?;

                entities.push(entity);
            } else if update_type == UpdateType::Preserve {
                let mut entity = get_entity_for_update(state, entity_index, update_type, delta)?;
                let send_table = get_send_table(state, entity.server_class)?;

                Self::read_update(&mut data, send_table, &mut entity.props, entity_index)?;

                entities.push(entity);
            } else if state.entity_classes.contains_key(&entity_index) {
                let entity = get_entity_for_update(state, entity_index, update_type, delta)?;
                entities.push(entity);
            } else {
                entities.push(PacketEntity {
                    server_class: 0.into(),
                    entity_index,
                    props: vec![],
                    in_pvs: false,
                    update_type,
                    serial_number: 0,
                    delay: None,
                    delta,
                    baseline_index: 0,
                });
            }
        }

        if delta.is_some() {
            while data.read()? {
                removed_entities.push(data.read_sized::<u32>(11)?.into())
            }
        }

        Ok(PacketEntitiesMessage {
            entities,
            removed_entities,
            max_entries,
            delta,
            base_line,
            updated_base_line,
        })
    }
}

impl Encode for PacketEntitiesMessage {
    fn encode(&self, stream: &mut BitWriteStream<LittleEndian>, state: &ParserState) -> Result<()> {
        self.max_entries.write_sized(stream, 11)?;
        self.delta.is_some().write(stream)?;
        if let Some(delta) = self.delta {
            delta.write(stream)?;
        }
        self.base_line.write_sized(stream, 1)?;
        self.entities.len().write_sized(stream, 11)?;

        stream.reserve_int(20, |stream| {
            self.updated_base_line.write(stream)?;

            let length_start = stream.bit_len();

            let mut last_index: i32 = -1;

            for entity in self.entities.iter() {
                let diff = entity.entity_index.0 as i32 - last_index - 1;
                write_bit_var(diff as u32, stream)?;
                last_index = entity.entity_index.0 as i32;

                entity.update_type.write(stream)?;

                let send_table = get_send_table(state, entity.server_class)?;
                match entity.update_type {
                    UpdateType::Enter => {
                        Self::write_enter(entity, stream, state)?;
                        Self::write_update(&entity.props, stream, send_table, entity.entity_index)?;
                    }
                    UpdateType::Preserve => {
                        Self::write_update(&entity.props, stream, send_table, entity.entity_index)?;
                    }
                    _ => {}
                }
            }

            if self.delta.is_some() {
                for removed in self.removed_entities.iter() {
                    true.write(stream)?;
                    removed.0.write_sized(stream, 11)?;
                }
                false.write(stream)?;
            }

            let length_end = stream.bit_len();

            Ok((length_end - length_start) as u64)
        })
    }
}

impl PacketEntitiesMessage {
    fn read_enter(
        stream: &mut Stream,
        entity_index: EntityId,
        state: &ParserState,
        baseline_index: usize,
        delta: Option<ServerTick>,
    ) -> Result<PacketEntity> {
        let bits = log_base2(state.server_classes.len()) + 1;
        let class_index: ClassId = stream.read_sized::<u16>(bits as usize)?.into();

        let serial = stream.read_sized(10)?;

        Ok(PacketEntity {
            server_class: class_index,
            entity_index,
            props: vec![],
            in_pvs: true,
            update_type: UpdateType::Enter,
            serial_number: serial,
            delay: None,
            delta,
            baseline_index,
        })
    }

    fn write_enter(
        entity: &PacketEntity,
        stream: &mut BitWriteStream<LittleEndian>,
        state: &ParserState,
    ) -> Result<()> {
        let bits = log_base2(state.server_classes.len()) + 1;
        u16::from(entity.server_class).write_sized(stream, bits as usize)?;
        entity.serial_number.write_sized(stream, 10)?;

        Ok(())
    }

    pub fn read_update(
        stream: &mut Stream,
        send_table: &SendTable,
        props: &mut Vec<SendProp>,
        entity_index: EntityId,
    ) -> Result<()> {
        let mut index: i32 = -1;

        #[cfg(feature = "trace")]
        trace!(entity_index = display(entity_index), "reading update");
        #[cfg(not(feature = "trace"))]
        let _ = entity_index;

        while stream.read()? {
            let diff: u32 = read_bit_var(stream)?;
            index = index.saturating_add(diff as i32).saturating_add(1);

            match send_table.flattened_props.get(index as usize) {
                Some(definition) => {
                    let value = SendPropValue::parse(stream, &definition.parse_definition)?;

                    #[cfg(feature = "trace")]
                    trace!(
                        entity_index = display(entity_index),
                        index = display(index),
                        value = debug(&value),
                        definition = display(definition.identifier),
                        "reading prop"
                    );
                    props.push(SendProp {
                        index: index as u32,
                        identifier: definition.identifier,
                        value,
                    });
                }
                None => {
                    return Err(ParseError::PropIndexOutOfBounds {
                        index,
                        prop_count: send_table.flattened_props.len(),
                        table: send_table.name.to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    pub fn write_update<'a, Props: IntoIterator<Item = &'a SendProp>>(
        props: Props,
        stream: &mut BitWriteStream<LittleEndian>,
        send_table: &SendTable,
        _entity_index: EntityId,
    ) -> Result<()> {
        let mut last_index: i32 = -1;

        let mut props: Vec<&SendProp> = props.into_iter().collect();
        props.sort_by(|a, b| a.index.cmp(&b.index));

        for prop in props {
            true.write(stream)?;

            let index = prop.index as usize;

            let definition =
                send_table
                    .flattened_props
                    .get(index)
                    .ok_or(ParseError::PropIndexOutOfBounds {
                        index: index as i32,
                        prop_count: send_table.flattened_props.len(),
                        table: send_table.name.to_string(),
                    })?;
            write_bit_var((index as i32 - last_index - 1) as u32, stream)?;
            last_index = index as i32;
            prop.value.encode(stream, &definition.parse_definition)?;
        }
        false.write(stream)?;
        Ok(())
    }
}

impl ParseBitSkip<'_> for PacketEntitiesMessage {
    fn parse_skip(stream: &mut Stream, _state: &ParserState) -> Result<()> {
        stream.skip_bits(11)?;
        if stream.read()? {
            stream.skip_bits(32)?;
        }
        stream.skip_bits(12)?;
        let length: u32 = stream.read_sized(20)?;
        stream
            .skip_bits(length as usize + 1)
            .map_err(ParseError::from)
    }
}

#[test]
fn test_packet_entitier_message_roundtrip() {
    use crate::demo::packet::datatable::{SendTable, SendTableName, ServerClass, ServerClassName};
    use crate::demo::sendprop::{FloatDefinition, SendPropDefinition, SendPropParseDefinition};

    let mut state = ParserState::new(24, |_| false, false);
    state.server_classes = vec![
        ServerClass {
            id: ClassId::from(0),
            name: ServerClassName::from("class1"),
            data_table: SendTableName::from("table1"),
        },
        ServerClass {
            id: ClassId::from(1),
            name: ServerClassName::from("class2"),
            data_table: SendTableName::from("table2"),
        },
    ];
    state.send_tables = vec![
        SendTable {
            name: SendTableName::from("table1"),
            needs_decoder: false,
            raw_props: vec![],
            flattened_props: vec![],
        },
        SendTable {
            name: SendTableName::from("table2"),
            needs_decoder: false,
            raw_props: vec![],
            flattened_props: vec![
                SendPropDefinition {
                    identifier: SendPropIdentifier::new("table2", "prop1"),
                    parse_definition: SendPropParseDefinition::Int {
                        changes_often: false,
                        bit_count: 8,
                    },
                },
                SendPropDefinition {
                    identifier: SendPropIdentifier::new("table2", "prop2"),
                    parse_definition: SendPropParseDefinition::String {
                        changes_often: false,
                    },
                },
                SendPropDefinition {
                    identifier: SendPropIdentifier::new("table2", "prop3"),
                    parse_definition: SendPropParseDefinition::Float {
                        changes_often: false,
                        definition: FloatDefinition::Coord,
                    },
                },
            ],
        },
    ];
    state
        .entity_classes
        .insert(EntityId::from(4u32), ClassId::from(1));
    crate::test_roundtrip_encode(
        PacketEntitiesMessage {
            entities: vec![],
            removed_entities: vec![],
            max_entries: 0,
            delta: None,
            base_line: 0,
            updated_base_line: false,
        },
        &state,
    );
    crate::test_roundtrip_encode(
        PacketEntitiesMessage {
            entities: vec![PacketEntity {
                server_class: ClassId::from(0),
                entity_index: Default::default(),
                props: vec![],
                in_pvs: true,
                update_type: UpdateType::Enter,
                serial_number: 0,
                delay: None,
                delta: None,
                baseline_index: 0,
            }],
            removed_entities: vec![],
            max_entries: 4,
            delta: None,
            base_line: 0,
            updated_base_line: false,
        },
        &state,
    );
    crate::test_roundtrip_encode(
        PacketEntitiesMessage {
            entities: vec![
                PacketEntity {
                    server_class: ClassId::from(0),
                    entity_index: EntityId::from(0u32),
                    props: vec![],
                    in_pvs: true,
                    update_type: UpdateType::Enter,
                    serial_number: 0,
                    delay: None,
                    delta: None,
                    baseline_index: 0,
                },
                PacketEntity {
                    server_class: ClassId::from(1),
                    entity_index: EntityId::from(4u32),
                    props: vec![
                        SendProp {
                            index: 0,
                            identifier: SendPropIdentifier::new("table2", "prop1"),
                            value: SendPropValue::Integer(4),
                        },
                        SendProp {
                            index: 2,
                            identifier: SendPropIdentifier::new("table2", "prop3"),
                            value: SendPropValue::Float(1.0),
                        },
                    ],
                    in_pvs: false,
                    update_type: UpdateType::Preserve,
                    serial_number: 0,
                    delay: None,
                    delta: None,
                    baseline_index: 0,
                },
                PacketEntity {
                    server_class: ClassId::from(1),
                    entity_index: EntityId::from(5u32),
                    delta: None,
                    baseline_index: 0,
                    props: vec![
                        SendProp {
                            index: 0,
                            identifier: SendPropIdentifier::new("table2", "prop1"),
                            value: SendPropValue::Integer(4),
                        },
                        SendProp {
                            index: 2,
                            identifier: SendPropIdentifier::new("table2", "prop3"),
                            value: SendPropValue::Float(1.0),
                        },
                    ],
                    in_pvs: true,
                    update_type: UpdateType::Enter,
                    serial_number: 0,
                    delay: None,
                },
            ],
            removed_entities: vec![],
            max_entries: 4,
            delta: None,
            base_line: 0,
            updated_base_line: false,
        },
        &state,
    );
}
