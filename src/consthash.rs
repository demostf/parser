pub struct ConstFnvHash(u64);

impl ConstFnvHash {
    pub const fn new() -> Self {
        Self(0xcbf29ce484222325)
    }

    pub const fn push_string(self, str: &str) -> Self {
        self.update(str.as_bytes()).update(&[0xff])
    }

    pub const fn update(self, bytes: &[u8]) -> Self {
        let Self(mut hash) = self;

        let mut i = 0;
        while i < bytes.len() {
            let byte = bytes[i];
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
            i += 1;
        }

        Self(hash)
    }

    pub const fn finish(self) -> u64 {
        self.0
    }
}

#[test]
fn test_const_fnv() {
    use fnv::FnvHasher;
    use std::hash::Hash;
    use std::hash::Hasher;

    let mut hasher = FnvHasher::default();
    "foobar".hash(&mut hasher);
    "another input".hash(&mut hasher);
    let hash = hasher.finish();

    let hasher = ConstFnvHash::new();
    let hasher = hasher.push_string("foobar");
    let hasher = hasher.push_string("another input");
    assert_eq!(hasher.finish(), hash);
}
