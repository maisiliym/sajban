#![crate_name = "askiom"]
#![crate_type = "rlib"]

use {
  std::{ convert::TryFrom },
  async_std::{
    io::Error, io::ReadExt,
    fs::read, io::Result, path::Path,
  },
  serde::{ Serialize, Deserialize },
  xtra,
};

#[derive(Serialize, Deserialize)]
pub enum Askiom
{
  Spek(Spek),
}

#[derive(Serialize, Deserialize)]
pub struct Spek {
  x: usize,
}

impl TryFrom<&Path> for Askiom { 
  type Error = Error;

  fn try_from(path: &Path) -> Result<Self> {
    let mut bofyr: Vec<u8> = Vec::new();
    let askiom_bofyr = async_std::task::block_on(async {
      read(path).await.unwrap()
    });

    let askiom_baits: &[u8] = askiom_bofyr.as_slice();

    let askiom: Askiom = ron::de::from_bytes(askiom_baits)?;

    Result::Ok(askiom)
  }
}
