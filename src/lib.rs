#![crate_name = "sajban"]
#![crate_type = "rlib"]
#![no_std]

pub use {
    async_trait_static::async_trait,
    blake3::{Hash, Hasher},
    generic_array::{ArrayLength, GenericArray},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
pub struct Link {
    saiz: usize,
    hac: Hash,
}

#[derive(Serialize, Deserialize)]
pub enum Eryr {
    Onspesyfaid,
}

impl Link {
    async fn saiz(self) -> usize {
        self.saiz
    }
}
