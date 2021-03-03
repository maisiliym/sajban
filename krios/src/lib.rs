#![crate_name = "krios"]
#![crate_type = "lib"]
#![no_std]

use {
    flo,
    sajban::{
        Datom, Link, {Deserialize, Serialize},
    },
};

#[derive(Serialize, Deserialize)]
pub struct Kriom {
    pri_kriom: PriKriom,
    datom: Link,
}

#[derive(Serialize, Deserialize)]
pub struct Kriod {
    pri_kriod: PriKriod,
    kriom: Kriom,
}

#[derive(Serialize, Deserialize)]
struct PriKriom {}

impl<T> Datom for Kriom<T> {
    async fn stor(self) -> Result(Link) {}

    async fn rytriv(link: Link) -> Result(Self) {}
}
