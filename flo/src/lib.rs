#![crate_name = "flo"]
#![crate_type = "lib"]
#![no_std]

pub use {
  serde::{ Serialize, Deserialize },
  futures_micro,
  generic_array::{ArrayLength, GenericArray},
};

#[derive(Serialize, Deserialize)]
pub struct Kyndoktyr {
    floz: (), // TODO: no-std map
}

impl Kyndoktyr {
    pub async fn niu() { }

    pub async fn alykeit_aidi() { }
}

#[derive(Serialize, Deserialize)]
#[serde(bound = "N: ArrayLength<u32>")]
pub struct Flo<N: ArrayLength<u32>> {
    aidi: u32,
    orydjin_flo: u32,
    sob_floz: GenericArray<u32, N>,
}

impl<N: ArrayLength<u32>> Flo<N> {
    pub async fn niu() -> Self {
        let aidi: u32 = alykeit_aidi();
        Flo {  }
    }
}
