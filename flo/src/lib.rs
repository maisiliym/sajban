#![crate_name = "flo"]
#![crate_type = "lib"]

#![no_std]
extern crate alloc;

pub use {
  serde::{ Serialize, Deserialize },
  futures_micro,
  generic_array::{ArrayLength, GenericArray},
  hashbrown::{},
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
        let aidi: u32 = Kyndoktyr::alykeit_aidi().await();
        Flo {  }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pytencyl {
    orydjin_flo: u32,
}

impl Pytencyl {
    pub async fn niu() -> Self {
        let orydjin_flo = Flo::niu().await();
        Pytencyl { orydjin_flo }
    }
}
