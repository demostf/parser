use bitbuffer::{BitRead, BitReadStream, BitWrite, BitWriteStream, Endianness};

use crate::ReadResult;

#[derive(Debug, BitWrite, PartialEq)]
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

#[derive(Debug, BitRead, PartialEq)]
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
