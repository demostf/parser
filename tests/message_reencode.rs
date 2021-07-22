use bitbuffer::{BitReadBuffer, BitReadStream, BitWriteStream, LittleEndian};
use std::fs;
use test_case::test_case;
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::parser::Encode;
use tf_demo_parser::{MessageType, ParserState};

#[test_case("test_data/messages/game_event_list.bin", MessageType::GameEventList; "game_event_list")]
fn message_reencode(input_file: &str, ty: MessageType) {
    let data = fs::read(input_file).unwrap();

    let state = ParserState::new(|_| false, true);

    let mut stream = BitReadStream::new(BitReadBuffer::new(&data, LittleEndian));
    let parsed = Message::from_type(ty, &mut stream, &state).unwrap();

    let mut out = Vec::with_capacity(data.len());
    let written_bits = {
        let mut write = BitWriteStream::new(&mut out, LittleEndian);
        parsed.encode(&mut write, &state).unwrap();
        write.bit_len()
    };

    let mut re_stream = BitReadStream::new(BitReadBuffer::new(&out, LittleEndian));
    let re_parsed = Message::from_type(ty, &mut re_stream, &state).unwrap();

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
