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
    sokyts: Sokyts,
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
pub struct Sokyts {
    one: Flo,
    two: Flo,
    three: Flo,
    four: Flo,
}

#[derive(Serialize, Deserialize)]
pub struct Flo {
    aidentyfaiyr: Aidentyfaiyr,
}

impl Flo {
    pub async fn niu() -> Self {
        let aidentyfaiyr: Aidentyfaiyr = Kyndoktyr::aidentyfai().await;
        Flo { aidentyfaiyr }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pytencyl {
    orydjin_flo: Aidentyfaiyr,
}

impl Pytencyl {
    pub async fn niu() -> Self {
        let orydjin_flo = Flo::niu().await;
        Pytencyl { orydjin_flo }
    }
}
