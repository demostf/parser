use bitstream_reader::{BitBuffer, BitStream, LittleEndian};

use tf_demo_parse_derive::Parse;
use tf_demo_parser::Parse;
use tf_demo_parser::ParserState;

#[derive(Parse, PartialEq, Debug)]
struct TestStruct {
    foo: u8,
    str: String,
    #[size = 2]
    truncated: String,
    bar: u16,
    float: f32,
    #[size = 3]
    asd: u8,
}

#[test]
fn test_read_struct() {
    let float: [u8; 4] = 12.5f32.to_bits().to_le_bytes();
    let bytes = &[
        12, 'h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, 0, 'f' as u8,
        'o' as u8, 'o' as u8, 0, float[0], float[1], float[2], float[3], 0b0101_0101
    ];
    let buffer = BitBuffer::new(bytes, LittleEndian);
    let mut stream = BitStream::new(buffer, None, None);
    let state = ParserState::new(&stream);
    assert_eq!(TestStruct {
        foo: 12,
        str: "hello".to_owned(),
        truncated: "fo".to_owned(),
        bar: 'o' as u16,
        float: 12.5,
        asd: 0b101,
    }, TestStruct::parse(&mut stream, &state).unwrap());
}
