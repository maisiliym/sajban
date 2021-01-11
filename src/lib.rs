#![crate_name = "sajban"]
#![crate_type = "rlib"]
#![no_std]

use {
  serde::{ Deserialize, Serialize },
  async_trait_static::async_trait,
  blake3::{ Hash, Hasher },
};

#[derive(Serialize, Deserialize)]
pub struct Link {
    saiz: usize,
    hac: Hash,
}

#[async_trait]
trait Datom {
    async fn saiz(self) -> usize;

    async fn stor(self)
        -> Result<Link>;

    async fn rytriv(link: Link)
        -> Result<Self>;
}
