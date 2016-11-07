use std::hash::Hasher;

pub struct Crc32cHasher {
    sum: u32
}

impl Default for Crc32cHasher {
    fn default() -> Crc32cHasher {
        Crc32cHasher {sum: 0}
    }
}

impl Hasher for Crc32cHasher {
    fn finish(&self) -> u64 {
        // repeat the 32-bit hash to get 64-bits ;-)
        let sum = self.sum as u64;
        (sum << 32) | sum
    }
    fn write(&mut self, bytes: &[u8]) {
        self.sum = crc32c_mem(self.sum, bytes)
    }
}

use std::hash::BuildHasherDefault;
use std::collections::{HashMap, HashSet};

/// A build for default Crc32 hashers
pub type Crc32cBuildHasher = BuildHasherDefault<Crc32cHasher>;

/// A `HashMap` using a default Crc32 hasher.
pub type Crc32cHashMap<K, V> = HashMap<K, V, Crc32cBuildHasher>;

/// A `HashSet` using a default Crc32 hasher.
pub type Crc32cHashSet<T> = HashSet<T, Crc32cBuildHasher>;

extern crate libc;
use libc::size_t;

extern {
    fn crc32c(crc: u32, data: *const u8, len: size_t) -> u32;
}

pub fn crc32c_mem(crc: u32, data: &[u8]) -> u32 {
    unsafe {
        crc32c(crc, data.as_ptr(), data.len() as size_t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(crc32c_mem(0, b""), 0x0);
        assert_eq!(crc32c_mem(0, b"a"), 0xc1d04330);
        assert_eq!(crc32c_mem(0, b"hello\n"), 0x353dd8be);
    }
}
