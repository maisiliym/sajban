#![crate_name = "sajban"]
#![crate_type = "rlib"]
#![no_std]

use {
  serde::{ Deserialize, Serialize },
  async_trait_static::async_trait,
  blake3::{ Hash, Hasher },
};

pub struct Link {
    saiz: Saiz,
    hac: Hash,
}

impl Link {
    async fn niu(datom: Datom)
    -> Result(Link) { }
}

#[async_trait]
trait Datom {
    async fn saiz(self) -> Saiz;

    async fn stor(self)
        -> Result(Link);

    async fn rytriv(link: Link)
        -> Result(Self);
}
