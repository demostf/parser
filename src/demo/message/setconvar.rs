use bitbuffer::{BitRead, BitReadStream, BitWrite, BitWriteStream, Endianness};
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::ReadResult;

#[cfg_attr(
    feature = "wasm",
    derive(wasm_typescript_definition::TypescriptDefinition)
)]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize)]
pub struct ConVar {
    key: String,
    value: String,
}

impl<E: Endianness> BitRead<'_, E> for ConVar {
    fn read(stream: &mut BitReadStream<'_, E>) -> ReadResult<Self> {
        let key = stream
            .read()
            .unwrap_or_else(|_| "Malformed cvar name".to_string());
        let value = stream
            .read()
            .unwrap_or_else(|_| "Malformed cvar value".to_string());
        Ok(ConVar { key, value })
    }
}

#[cfg_attr(
    feature = "wasm",
    derive(wasm_typescript_definition::TypescriptDefinition)
)]
#[derive(Debug, BitRead, PartialEq, Serialize, Deserialize)]
pub struct SetConVarMessage {
    length: u8,
    #[size = "length"]
    vars: Vec<ConVar>,
}

impl<E: Endianness> BitWrite<E> for SetConVarMessage {
    fn write(&self, stream: &mut BitWriteStream<E>) -> ReadResult<()> {
        self.length.write(stream)?;
        self.vars.write(stream)
    }
}
#[test]
fn test_set_con_var_roundtrip() {
    crate::test_roundtrip_write(SetConVarMessage {
        length: 0,
        vars: Vec::new(),
    });
    crate::test_roundtrip_write(SetConVarMessage {
        length: 2,
        vars: vec![
            ConVar {
                key: "foo1".to_string(),
                value: "bar1".to_string(),
            },
            ConVar {
                key: "foo2".to_string(),
                value: "bar2".to_string(),
            },
        ],
    });
}
