use crate::demo::data::DemoTick;
use crate::demo::message::packetentities::EntityId;
use crate::demo::message::packetentities::PacketEntity;
use crate::demo::message::{Message, MessageType};
use crate::demo::packet::datatable::ClassId;
use crate::demo::packet::stringtable::StringTableEntry;
use crate::demo::parser::analyser::UserInfo;
use crate::demo::parser::gamestateanalyser::UserId;
use crate::demo::parser::handler::{BorrowMessageHandler, MessageHandler};
use crate::{ParserState, ReadResult, Stream};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/**
 * An analyzer that extracts player scoreboard information to get the stats for every player by the
 * end of the demo.  Essentially, this will capture all the information that would appear on the
 * scoreboard for every player if they took a snapshot at the time the demo finishes (such as the end
 * of a match or round).
 */
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerSummaryAnalyzer {
    state: PlayerSummaryState,
    user_id_map: HashMap<EntityId, UserId>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct PlayerSummary {
    pub points: u32,
    pub kills: u32,
    pub assists: u32,
    pub deaths: u32,
    pub buildings_destroyed: u32,
    pub captures: u32,
    pub defenses: u32,
    pub dominations: u32,
    pub revenges: u32,
    pub ubercharges: u32,
    pub headshots: u32,
    pub teleports: u32,
    pub healing: u32,
    pub backstabs: u32,
    pub bonus_points: u32,
    pub support: u32,
    pub damage_dealt: u32,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerSummaryState {
    pub player_summaries: HashMap<UserId, PlayerSummary>,
    pub users: BTreeMap<UserId, UserInfo>,
}

impl MessageHandler for PlayerSummaryAnalyzer {
    type Output = PlayerSummaryState;

    fn does_handle(message_type: MessageType) -> bool {
        matches!(message_type, MessageType::PacketEntities)
    }

    fn handle_message(&mut self, message: &Message, _tick: DemoTick, parser_state: &ParserState) {
        match message {
            Message::PacketEntities(message) => {
                for entity in message.entities.iter() {
                    self.handle_packet_entity(&entity, parser_state);
                }
            }
            _ => {}
        }
    }

    fn into_output(self, _parser_state: &ParserState) -> <Self as MessageHandler>::Output {
        self.state
    }

    fn handle_string_entry(
        &mut self,
        table: &str,
        index: usize,
        entry: &StringTableEntry,
        _parser_state: &ParserState,
    ) {
        if table == "userinfo" {
            let _ = self.parse_user_info(
                index,
                entry.text.as_ref().map(|s| s.as_ref()),
                entry.extra_data.as_ref().map(|data| data.data.clone()),
            );
        }
    }
}

impl BorrowMessageHandler for PlayerSummaryAnalyzer {
    fn borrow_output(&self, _state: &ParserState) -> &Self::Output {
        &self.state
    }
}

/**
 * Helper function to make processing integer properties easier.
 *
 * parse_integer_prop(packet, "DT_TFPlayerScoringDataExclusive", "m_iPoints", |points| { println!("Scored {} points", points) });
 */
fn parse_integer_prop<F>(
    packet: &PacketEntity,
    table: &str,
    name: &str,
    parser_state: &ParserState,
    handler: F,
) where
    F: FnOnce(u32),
{
    use crate::demo::sendprop::SendPropValue;

    match packet.get_prop_by_name(table, name, parser_state) {
        Some(prop) => {
            match prop.value {
                SendPropValue::Integer(val) => handler(val as u32),
                _ => {} // not an integer, ignore
            }
        }
        None => {} // the packet doesn't have this property
    }
}

impl PlayerSummaryAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    fn handle_packet_entity(&mut self, packet: &PacketEntity, parser_state: &ParserState) {
        use crate::demo::sendprop::SendPropValue;

        // println!("Known server classes: {:?}", parser_state.server_classes);

        if let Some(class) = parser_state
            .server_classes
            .get(<ClassId as Into<usize>>::into(packet.server_class))
        {
            // println!("Got a {} data packet: {:?}", class.name, packet);
            match class.name.as_str() {
                "CTFPlayer" => {
                    match self.user_id_map.get(&packet.entity_index) {
                        Some(user_id) => {
                            let summaries = &mut self.state.player_summaries;
                            let player_summary = summaries.entry(*user_id).or_default();

                            // Extract scoreboard information, if present, and update the player's summary accordingly
                            // NOTE: Multiple DT_TFPlayerScoringDataExclusive structures may be present - one for the entire match,
                            //       and one for just the current round.  Since we're only interested in the overall match scores,
                            //       we need to ignore the round-specific values.  Fortunately, this is easy - just ignore the
                            //       lesser value (if multiple values are present), since none of these scores are able to decrement.

                            /*
                             * Member: m_iCaptures (offset 4) (type integer) (bits 10) (Unsigned)
                             * Member: m_iDefenses (offset 8) (type integer) (bits 10) (Unsigned)
                             * Member: m_iKills (offset 12) (type integer) (bits 10) (Unsigned)
                             * Member: m_iDeaths (offset 16) (type integer) (bits 10) (Unsigned)
                             * Member: m_iSuicides (offset 20) (type integer) (bits 10) (Unsigned)
                             * Member: m_iDominations (offset 24) (type integer) (bits 10) (Unsigned)
                             * Member: m_iRevenge (offset 28) (type integer) (bits 10) (Unsigned)
                             * Member: m_iBuildingsBuilt (offset 32) (type integer) (bits 10) (Unsigned)
                             * Member: m_iBuildingsDestroyed (offset 36) (type integer) (bits 10) (Unsigned)
                             * Member: m_iHeadshots (offset 40) (type integer) (bits 10) (Unsigned)
                             * Member: m_iBackstabs (offset 44) (type integer) (bits 10) (Unsigned)
                             * Member: m_iHealPoints (offset 48) (type integer) (bits 20) (Unsigned)
                             * Member: m_iInvulns (offset 52) (type integer) (bits 10) (Unsigned)
                             * Member: m_iTeleports (offset 56) (type integer) (bits 10) (Unsigned)
                             * Member: m_iDamageDone (offset 60) (type integer) (bits 20) (Unsigned)
                             * Member: m_iCrits (offset 64) (type integer) (bits 10) (Unsigned)
                             * Member: m_iResupplyPoints (offset 68) (type integer) (bits 10) (Unsigned)
                             * Member: m_iKillAssists (offset 72) (type integer) (bits 12) (Unsigned)
                             * Member: m_iBonusPoints (offset 76) (type integer) (bits 10) (Unsigned)
                             * Member: m_iPoints (offset 80) (type integer) (bits 10) (Unsigned)
                             *
                             * NOTE: support points aren't included here, but is equal to the sum of m_iHealingAssist and m_iDamageAssist
                             * TODO: pull data for support points
                             */
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iCaptures",
                                parser_state,
                                |captures| {
                                    if captures > player_summary.captures {
                                        player_summary.captures = captures;
                                    }
                                },
                            );
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iDefenses",
                                parser_state,
                                |defenses| {
                                    if defenses > player_summary.defenses {
                                        player_summary.defenses = defenses;
                                    }
                                },
                            );
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iKills",
                                parser_state,
                                |kills| {
                                    if kills > player_summary.kills {
                                        // TODO: This might not be accruate.  Tested with a demo file with 89 kills (88 on the scoreboard),
                                        // but only a 83 were reported in the scoring data.
                                        player_summary.kills = kills;
                                    }
                                },
                            );
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iDeaths",
                                parser_state,
                                |deaths| {
                                    if deaths > player_summary.deaths {
                                        player_summary.deaths = deaths;
                                    }
                                },
                            );
                            // ignore m_iSuicides
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iDominations",
                                parser_state,
                                |dominations| {
                                    if dominations > player_summary.dominations {
                                        player_summary.dominations = dominations;
                                    }
                                },
                            );
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iRevenge",
                                parser_state,
                                |revenges| {
                                    if revenges > player_summary.revenges {
                                        player_summary.revenges = revenges;
                                    }
                                },
                            );
                            // ignore m_iBuildingsBuilt
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iBuildingsDestroyed",
                                parser_state,
                                |buildings_destroyed| {
                                    if buildings_destroyed > player_summary.buildings_destroyed {
                                        player_summary.buildings_destroyed = buildings_destroyed;
                                    }
                                },
                            );
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iHeadshots",
                                parser_state,
                                |headshots| {
                                    if headshots > player_summary.headshots {
                                        player_summary.headshots = headshots;
                                    }
                                },
                            );
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iBackstabs",
                                parser_state,
                                |backstabs| {
                                    if backstabs > player_summary.backstabs {
                                        player_summary.backstabs = backstabs;
                                    }
                                },
                            );
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iHealPoints",
                                parser_state,
                                |healing| {
                                    if healing > player_summary.healing {
                                        player_summary.healing = healing;
                                    }
                                },
                            );
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iInvulns",
                                parser_state,
                                |ubercharges| {
                                    if ubercharges > player_summary.ubercharges {
                                        player_summary.ubercharges = ubercharges;
                                    }
                                },
                            );
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iTeleports",
                                parser_state,
                                |teleports| {
                                    if teleports > player_summary.teleports {
                                        player_summary.teleports = teleports;
                                    }
                                },
                            );
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iDamageDone",
                                parser_state,
                                |damage_dealt| {
                                    if damage_dealt > player_summary.damage_dealt {
                                        player_summary.damage_dealt = damage_dealt;
                                    }
                                },
                            );
                            // ignore m_iCrits
                            // ignore m_iResupplyPoints
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iKillAssists",
                                parser_state,
                                |assists| {
                                    if assists > player_summary.assists {
                                        player_summary.assists = assists;
                                    }
                                },
                            );
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iBonusPoints",
                                parser_state,
                                |bonus_points| {
                                    if bonus_points > player_summary.bonus_points {
                                        player_summary.bonus_points = bonus_points;
                                    }
                                },
                            );
                            parse_integer_prop(
                                packet,
                                "DT_TFPlayerScoringDataExclusive",
                                "m_iPoints",
                                parser_state,
                                |points| {
                                    if points > player_summary.points {
                                        player_summary.points = points;
                                    }
                                },
                            );
                        }
                        None => {
                            // Player entity likely spawned before the player was assigned to it?
                            // This can rarely happen, but doesn't seem to affect the end result
                        }
                    }
                }
                "CTFPlayerResource" => {
                    // Player summaries - including entity IDs!
                    // look for props like m_iUserID.<entity_id> = <user_id>
                    // for example, `m_iUserID.024 = 2523` means entity 24 is user 2523
                    for i in 0..33 {
                        // 0 to 32, inclusive (1..33 might also work, not sure if there's a user 0 or not).  Not exhaustive and doesn't work for servers with > 32 players
                        match packet.get_prop_by_name(
                            "m_iUserID",
                            format!("{:0>3}", i).as_str(),
                            parser_state,
                        ) {
                            Some(prop) => {
                                match prop.value {
                                    SendPropValue::Integer(x) => {
                                        let entity_id = EntityId::from(i as u32);
                                        let user_id = UserId::from(x as u32);
                                        self.user_id_map.insert(entity_id, user_id);
                                    }
                                    _ => {
                                        // These properties should all be integers...
                                    }
                                }
                            }
                            None => {} // ignore, no property for this entity was included
                        }
                    }
                }
                _other => {
                    // Don't care
                }
            }
        }
    }

    fn parse_user_info(
        &mut self,
        index: usize,
        text: Option<&str>,
        data: Option<Stream>,
    ) -> ReadResult<()> {
        if let Some(user_info) =
            crate::demo::data::UserInfo::parse_from_string_table(index as u16, text, data)?
        {
            self.state
                .users
                .entry(user_info.player_info.user_id.into())
                .and_modify(|info| {
                    info.entity_id = user_info.entity_id;
                })
                .or_insert_with(|| user_info.into());
        }

        Ok(())
    }
}
