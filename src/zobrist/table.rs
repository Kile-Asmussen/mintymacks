use std::{
    collections::{HashMap, VecDeque, hash_map::Entry},
    hash::{BuildHasher, Hasher},
};

use indexmap::IndexMap;

use crate::zobrist::ZobHash;

#[derive(Debug, Clone)]
pub struct ZobTable<T>(HashMap<ZobHash, T, ZobHashing>, VecDeque<ZobHash>);

impl<T: Default + Clone + Copy> ZobTable<T> {
    fn per_megabyte() -> usize {
        (1000 * 1024) / std::mem::size_of::<(ZobHash, ZobHash, T)>()
    }

    fn megabytes(mb: usize) -> Self {
        let mb: usize = 1.max(mb);
        let entries = mb * Self::per_megabyte();
        ZobTable(
            HashMap::with_capacity_and_hasher(entries, ZobHashing),
            VecDeque::with_capacity(entries),
        )
    }

    pub fn insert(&mut self, k: ZobHash, v: T) {
        if self.0.len() == self.0.capacity() || self.1.len() == self.1.capacity() {
            self.0.remove(&self.1.pop_front().unwrap_or(0));
        }
        self.0.insert(k, v);
        self.1.push_back(k);
    }

    pub fn get(&self, k: ZobHash) -> Option<T> {
        self.0.get(&k).map(|x| *x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZobHashing;

impl BuildHasher for ZobHashing {
    type Hasher = ZobHasher;

    fn build_hasher(&self) -> Self::Hasher {
        ZobHasher(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ZobHasher(pub ZobHash);

impl Hasher for ZobHasher {
    fn finish(&self) -> ZobHash {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        unimplemented!()
    }

    fn write_u8(&mut self, i: u8) {
        unimplemented!()
    }

    fn write_u16(&mut self, i: u16) {
        unimplemented!()
    }

    fn write_u32(&mut self, i: u32) {
        unimplemented!()
    }

    fn write_u64(&mut self, i: u64) {
        self.0 ^= i;
    }

    fn write_u128(&mut self, i: u128) {
        unimplemented!()
    }

    fn write_usize(&mut self, i: usize) {
        unimplemented!()
    }

    fn write_i8(&mut self, i: i8) {
        unimplemented!()
    }

    fn write_i16(&mut self, i: i16) {
        unimplemented!()
    }

    fn write_i32(&mut self, i: i32) {
        unimplemented!()
    }

    fn write_i64(&mut self, i: i64) {
        unimplemented!()
    }

    fn write_i128(&mut self, i: i128) {
        unimplemented!()
    }

    fn write_isize(&mut self, i: isize) {
        unimplemented!()
    }

    fn write_length_prefix(&mut self, len: usize) {
        unimplemented!()
    }

    fn write_str(&mut self, s: &str) {
        unimplemented!()
    }
}
