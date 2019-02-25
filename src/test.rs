use crate::*;
use std::fmt::Debug;
use std::fs;

//pub fn test_parse<'a, T: Parse<'a> + Debug + PartialEq>(path: &str, expected: T, offset: usize) {
//    let file = fs::read(path).expect("Unable to read file");
//    let demo = Demo::new(file);
//    let stream: Stream = demo.get_stream();
//    let mut parser = DemoParser::new(stream);
//    let _ = parser.set_stream_pos(offset);
//    let result = parser.read().unwrap();
//    assert_eq!(expected, result);
//    let end_pos = parser.stream_pos();
//    let _ = parser.set_stream_pos(offset);
//    let _ = parser.skip::<T>();
//    assert_eq!(parser.stream_pos(), end_pos);
//}
