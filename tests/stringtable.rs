use bitbuffer::{BitReadBuffer, BitReadStream, BitWriteStream, LittleEndian};
use std::fs;
use test_case::test_case;
use tf_demo_parser::demo::message::stringtable::{
    parse_string_table_update, write_string_table_update, StringTableMeta,
};
use tf_demo_parser::demo::packet::stringtable::FixedUserDataSize;

#[test_case("test_data/string_tables/decalprecache.bin", "test_data/string_tables/decalprecache_meta.json"; "decalprecache")]
#[test_case("test_data/string_tables/downloadables.bin", "test_data/string_tables/downloadables_meta.json"; "downloadables")]
#[test_case("test_data/string_tables/DynamicModels.bin", "test_data/string_tables/DynamicModels_meta.json"; "DynamicModels")]
#[test_case("test_data/string_tables/EffectDispatch.bin", "test_data/string_tables/EffectDispatch_meta.json"; "EffectDispatch")]
#[test_case("test_data/string_tables/GameRulesCreation.bin", "test_data/string_tables/GameRulesCreation_meta.json"; "GameRulesCreation")]
#[test_case("test_data/string_tables/genericprecache.bin", "test_data/string_tables/genericprecache_meta.json"; "genericprecache")]
#[test_case("test_data/string_tables/InfoPanel.bin", "test_data/string_tables/InfoPanel_meta.json"; "InfoPanel")]
// #[test_case("test_data/string_tables/instancebaseline.bin", "test_data/string_tables/instancebaseline_meta.json"; "instancebaseline")]
#[test_case("test_data/string_tables/lightstyles.bin", "test_data/string_tables/lightstyles_meta.json"; "lightstyles")]
#[test_case("test_data/string_tables/Materials.bin", "test_data/string_tables/Materials_meta.json"; "Materials")]
#[test_case("test_data/string_tables/modelprecache.bin", "test_data/string_tables/modelprecache_meta.json"; "modelprecache")]
#[test_case("test_data/string_tables/ParticleEffectNames.bin", "test_data/string_tables/ParticleEffectNames_meta.json"; "ParticleEffectNames")]
// #[test_case("test_data/string_tables/Scenes.bin", "test_data/string_tables/Scenes_meta.json"; "Scenes")]
#[test_case("test_data/string_tables/server_query_info.bin", "test_data/string_tables/server_query_info_meta.json"; "server_query_info")]
#[test_case("test_data/string_tables/ServerMapCycle.bin", "test_data/string_tables/ServerMapCycle_meta.json"; "ServerMapCycle")]
#[test_case("test_data/string_tables/ServerMapCycleMvM.bin", "test_data/string_tables/ServerMapCycleMvM_meta.json"; "ServerMapCycleMvM")]
#[test_case("test_data/string_tables/ServerPopFiles.bin", "test_data/string_tables/ServerPopFiles_meta.json"; "ServerPopFiles")]
#[test_case("test_data/string_tables/soundprecache.bin", "test_data/string_tables/soundprecache_meta.json"; "soundprecache")]
#[test_case("test_data/string_tables/userinfo.bin", "test_data/string_tables/userinfo_meta.json"; "userinfo")]
#[test_case("test_data/string_tables/VguiScreen.bin", "test_data/string_tables/VguiScreen_meta.json"; "VguiScreen")]
fn string_table_reencode(input_file: &str, meta_file: &str) {
    let data = fs::read(input_file).unwrap();
    let meta: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(meta_file).unwrap()).unwrap();
    let count = meta["count"].as_u64().unwrap() as u16;
    let max_entries = meta["max_entries"].as_u64().unwrap();
    let fixed_userdata_size =
        meta["fixed_userdata_size"]
            .as_object()
            .map(|size| FixedUserDataSize {
                size: size["size"].as_u64().unwrap() as u16,
                bits: size["bits"].as_u64().unwrap() as u8,
            });
    let table_meta = StringTableMeta {
        max_entries: max_entries as u16,
        fixed_userdata_size,
    };
    let mut stream = BitReadStream::new(BitReadBuffer::new(&data, LittleEndian));
    let parsed = parse_string_table_update(&mut stream, &table_meta, count).unwrap();

    let mut out = Vec::with_capacity(data.len());
    let written_bits = {
        let mut write = BitWriteStream::new(&mut out, LittleEndian);
        write_string_table_update(&parsed, &mut write, &table_meta).unwrap();
        write.bit_len()
    };

    let mut re_stream = BitReadStream::new(BitReadBuffer::new(&out, LittleEndian));
    let re_parsed = parse_string_table_update(&mut re_stream, &table_meta, count).unwrap();

    assert_eq!(written_bits, stream.pos());
    assert_eq!(parsed, re_parsed);

    assert_eq!(data.len(), out.len());
    if data.len() > 16 {
        pretty_assertions::assert_eq!(&data[0..data.len() - 8], &out[0..out.len() - 8]);
        pretty_assertions::assert_eq!(&data[data.len() - 8..], &out[out.len() - 8..]);
    } else {
        pretty_assertions::assert_eq!(data, out);
    }
}
