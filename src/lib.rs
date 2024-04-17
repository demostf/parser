#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![cfg_attr(not(test), deny(clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::indexing_slicing))]
#![cfg_attr(not(test), deny(clippy::panic))]

use std::ffi::{c_char, CStr, CString};
use std::fs;
pub use bitbuffer::Result as ReadResult;
use serde::{Deserialize, Serialize};

pub use crate::demo::{
    message::MessageType,
    parser::{
        DemoParser, GameEventError, MatchState, MessageTypeAnalyser, Parse, ParseError,
        ParserState, Result,
    },
    Demo, Stream,
};
use crate::demo::header::Header;

#[cfg(feature = "codegen")]
pub mod codegen;
pub(crate) mod consthash;
pub mod demo;
pub(crate) mod nullhasher;

#[cfg(test)]
#[track_caller]
fn test_roundtrip_write<
    'a,
    T: bitbuffer::BitRead<'a, bitbuffer::LittleEndian>
        + bitbuffer::BitWrite<bitbuffer::LittleEndian>
        + std::fmt::Debug
        + std::cmp::PartialEq,
>(
    val: T,
) {
    let mut data = Vec::with_capacity(128);
    use bitbuffer::{BitReadBuffer, BitReadStream, BitWriteStream, LittleEndian};
    let pos = {
        let mut stream = BitWriteStream::new(&mut data, LittleEndian);
        val.write(&mut stream).unwrap();
        stream.bit_len()
    };

    let mut read = BitReadStream::new(BitReadBuffer::new_owned(data, LittleEndian));
    assert_eq!(
        val,
        read.read().unwrap(),
        "Failed to assert the parsed message is equal to the original"
    );
    assert_eq!(
        pos,
        read.pos(),
        "Failed to assert that all encoded bits ({}) are used for decoding ({})",
        pos,
        read.pos()
    );
}

#[cfg(test)]
#[track_caller]
fn test_roundtrip_encode<
    'a,
    T: Parse<'a> + crate::demo::parser::Encode + std::fmt::Debug + std::cmp::PartialEq,
>(
    val: T,
    state: &ParserState,
) {
    let mut data = Vec::with_capacity(128);
    use bitbuffer::{BitReadBuffer, BitReadStream, BitWriteStream, LittleEndian};
    let pos = {
        let mut stream = BitWriteStream::new(&mut data, LittleEndian);
        val.encode(&mut stream, state).unwrap();
        stream.bit_len()
    };

    let mut read = BitReadStream::new(BitReadBuffer::new_owned(data, LittleEndian));
    pretty_assertions::assert_eq!(
        val,
        T::parse(&mut read, state).unwrap(),
        "Failed to assert the parsed message is equal to the original"
    );
    pretty_assertions::assert_eq!(
        pos,
        read.pos(),
        "Failed to assert that all encoded bits ({}) are used for decoding ({})",
        pos,
        read.pos()
    );
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonDemo {
    header: Header,
    #[serde(flatten)]
    state: MatchState,
}

#[no_mangle]
pub extern "C" fn analyze_demo(path: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(path) };
    let str_slice = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let file = match fs::read(str_slice) {
        Ok(f) => f,
        Err(_) => return std::ptr::null_mut(),
    };

    let demo = Demo::new(&file);
    let parser = DemoParser::new(demo.get_stream());
    let (header, state) = match parser.parse() {
        Ok(results) => results,
        Err(_) => return std::ptr::null_mut(),
    };

    let demo_json = JsonDemo { header, state };
    let result = match serde_json::to_string(&demo_json) {
        Ok(json) => json,
        Err(_) => return std::ptr::null_mut(),
    };

    // Convert the Rust string to a C string to return
    let c_string = match CString::new(result) {
        Ok(c_str) => c_str,
        Err(_) => return std::ptr::null_mut(),
    };

    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        // Reclaim the CString to properly deallocate memory
        if !s.is_null() {
            let _ = CString::from_raw(s);
        }
    }
}
