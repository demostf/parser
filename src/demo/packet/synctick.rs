use bitbuffer::{BitRead, BitWrite};
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(
    feature = "wasm",
    derive(wasm_typescript_definition::TypescriptDefinition)
)]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncTickPacket {
    pub tick: u32,
}
