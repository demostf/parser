use bitbuffer::{BitRead, BitWrite};
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(
    feature = "wasm",
    derive(wasm_typescript_definition::TypescriptDefinition)
)]
#[derive(Debug, BitRead, BitWrite, PartialEq, Serialize, Deserialize)]
pub struct StopPacket {
    #[size = 24]
    pub tick: u32,
}
