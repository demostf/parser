use fnv::FnvHashMap;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use tf_demo_parser::demo::packet::datatable::{ParseSendTable, SendTableName, ServerClass};
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::demo::sendprop::{SendPropIdentifier, SendPropName};
use tf_demo_parser::{Demo, DemoParser, MessageType, ParserState};

struct PropInfo {
    identifier: SendPropIdentifier,
    table_name: String,
    prop_name: String,
}

#[derive(Default)]
struct PropAnalyzer {
    prop_names: FnvHashMap<SendPropIdentifier, (SendTableName, SendPropName)>,
}

impl MessageHandler for PropAnalyzer {
    type Output = Vec<PropInfo>;

    fn does_handle(message_type: MessageType) -> bool {
        matches!(message_type, MessageType::PacketEntities)
    }

    fn handle_data_tables(
        &mut self,
        parse_tables: &[ParseSendTable],
        _server_classes: &[ServerClass],
        _state: &ParserState,
    ) {
        let mut numeric_tables: FnvHashMap<String, usize> = HashMap::default();
        for table in parse_tables {
            if table.props.iter().any(|prop| {
                prop.name == "lengthproxy" || prop.name.as_str().starts_with("lengthprop")
            }) {
                continue;
            }
            for prop_def in &table.props {
                self.prop_names.insert(
                    prop_def.identifier(),
                    (table.name.clone(), prop_def.name.clone()),
                );
                let name = prop_def.name.as_str();
                if name.len() == 3 && table.name.as_str().len() > 3 {
                    if let Ok(_) = name.parse::<u8>() {
                        let size = match table.name.as_str() {
                            "m_nNextMapVoteOptions" => 3,
                            "m_nStreaks"
                            | "m_nNumNodeHillData"
                            | "m_nModelIndexOverrides"
                            | "m_nMinigameTeamScore"
                            | "m_iTeamBaseIcons"
                            | "m_iTeam"
                            | "m_iNumTeamMembers"
                            | "m_hProps"
                            | "m_flNextRespawnWave"
                            | "m_flEncodedController"
                            | "m_eWinningMethod"
                            | "m_chPoseIndex"
                            | "m_bTrackAlarm"
                            | "m_bTeamReady"
                            | "m_bTeamCanCap"
                            | "m_TeamRespawnWaveTimes" => 4,
                            "m_nVoteOptionCount" => 5,
                            "m_iWarnOnCap"
                            | "m_iTeamInZone"
                            | "m_iOwner"
                            | "m_iControlPointParents"
                            | "m_iCappingTeam"
                            | "m_iCPGroup"
                            | "m_hControlPointEnts"
                            | "m_flUnlockTimes"
                            | "m_flPathDistance"
                            | "m_flLazyCapPerc"
                            | "m_flCPTimerTimes"
                            | "m_bInMiniRound"
                            | "m_bCPLocked"
                            | "m_bCPIsVisible"
                            | "m_bCPCapRateScalesWithPlayers"
                            | "m_bBlocked" => 8,
                            "m_nAttachIndex" | "m_hAttachEntity" => 10,
                            "m_nMannVsMachineWaveClassFlags"
                            | "m_nMannVsMachineWaveClassFlags2"
                            | "m_nMannVsMachineWaveClassCounts2"
                            | "m_nMannVsMachineWaveClassCounts"
                            | "m_bMannVsMachineWaveClassActive2"
                            | "m_bMannVsMachineWaveClassActive" => 12,
                            "m_chCurrentSlideLists" => 16,
                            "m_bHillIsDownhill" => 20,
                            "m_chAreaPortalBits" | "m_chAreaBits" => 24,
                            "m_hMyWeapons" => 48,
                            "m_iTeamReqCappers" | "m_iTeamOverlays" | "m_iTeamIcons" => 8 * 8,
                            "m_flexWeight" | "m_flPoseParameter" => 96,
                            _ => 65,
                        };
                        numeric_tables.insert(table.name.to_string(), size);
                    }
                }
            }
        }
        for (table, size) in numeric_tables {
            for num in 0..=size {
                let prop_name = SendPropName::from(format!("{:03}", num));
                self.prop_names.insert(
                    SendPropIdentifier::new(&table, prop_name.as_str()),
                    (table.clone().into(), prop_name),
                );
            }
        }
    }

    fn into_output(self, _state: &ParserState) -> Self::Output {
        let mut props: Vec<_> = self
            .prop_names
            .into_iter()
            .map(|(identifier, (table_name, prop_name))| PropInfo {
                identifier,
                table_name: table_name.to_string(),
                prop_name: prop_name.to_string(),
            })
            .collect();
        props.sort_by(|a, b| {
            a.table_name
                .cmp(&b.table_name)
                .then(a.prop_name.cmp(&b.prop_name))
        });
        props
    }
}

pub fn generate_prop_names(demo: Demo) -> TokenStream {
    let (_, props) = DemoParser::new_with_analyser(demo.get_stream(), PropAnalyzer::default())
        .parse()
        .unwrap();

    let imports = quote!(
        use crate::demo::sendprop::SendPropIdentifier;
    );

    let matches = props.into_iter().map(|prop| {
        let identifier: u64 = prop.identifier.into();
        let table_name = prop.table_name;
        let prop_name = prop.prop_name;
        quote! {
            #identifier => Some((#table_name, #prop_name))
        }
    });

    quote!(
        #imports

        pub fn get_prop_names(identifier: SendPropIdentifier) -> Option<(&'static str, &'static str)> {
            let identifier: u64 = identifier.into();
            match identifier {
                #(#matches,)*
                _ => None,
            }
        }
    )
}
