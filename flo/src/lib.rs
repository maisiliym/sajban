#![crate_name = "flo"]
#![crate_type = "lib"]
#![no_std]
extern crate alloc;

pub use {
    futures_micro,
    sajban::{ArrayLength, GenericArray},
    sajban::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
pub struct Kyndoktyr {
    floz: (), // TODO: no-std map
}

impl Kyndoktyr {
    pub async fn niu() {}
    pub async fn alykeit_aidi() {}
}

/* TODO('Derive from number of cpu-cores') */
#[derive(Serialize, Deserialize)]
pub enum Sokyt {
    One,
    Two,
    Three,
    Four,
}

#[derive(Serialize, Deserialize)]
#[serde(bound = "N: ArrayLength<u32>")]
pub struct Flo<N: ArrayLength<u32>> {
    sokyt: Sokyt,
    orydjin_flo: Sokyt,
    sob_floz: GenericArray<Sokyt, N>,
}

impl<N: ArrayLength<u32>> Flo<N> {
    pub async fn niu() -> Self {
        let aidi: u32 = Kyndoktyr::alykeit_aidi().await;
        Flo {}
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pytencyl {
    orydjin_flo: u32,
}

impl Pytencyl {
    pub async fn niu() -> Self {
        let orydjin_flo = Flo::niu().await;
        Pytencyl { orydjin_flo }
    }
}
