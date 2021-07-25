use serde::{Deserialize, Serialize};
use std::hash::{BuildHasher, Hasher};

/// A dummy hasher that maps simply returns the hashed u64
///
/// trying to hash anything but a u64 will result in a panic
pub struct NullHasher {
    data: u64,
}

impl Hasher for NullHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.data
    }

    #[inline]
    fn write(&mut self, _msg: &[u8]) {
        panic!("can only hash u64,u32,u16");
    }

    #[inline]
    fn write_u16(&mut self, data: u16) {
        self.data = data as u64
    }

    #[inline]
    fn write_u32(&mut self, data: u32) {
        self.data = data as u64
    }

    #[inline]
    fn write_u64(&mut self, data: u64) {
        self.data = data;
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct NullHasherBuilder;

impl BuildHasher for NullHasherBuilder {
    type Hasher = NullHasher;

    fn build_hasher(&self) -> Self::Hasher {
        NullHasher { data: 0 }
    }
}
