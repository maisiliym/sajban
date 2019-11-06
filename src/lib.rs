#![crate_name = "sajban"]
#![crate_type = "rlib"]
#![no_std]

use {
  serde::{ Deserialize, Serialize },
  async_trait_static::async_trait,
};

#[derive(Serialize, Deserialize)]
enum Datom { }

#[async_trait]
trait Datom {
  async fn saiz() -> Saiz;
}
