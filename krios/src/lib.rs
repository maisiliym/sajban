#![crate_name = "krios"]
#![crate_type = "lib"]
#![no_std]

use {
    sajban::{ Datom, Link, },
};

pub trait Kriom {
    async fn modyfai(self, modyfaiyr: Modyfaiyr) -> Result(Self);    
}

struct PriKriom { }

impl<T> Datom for Kriom<T> {
    async fn stor(self)
        -> Result(Link) { }

    async fn rytriv(link: Link)
        -> Result(Self) { }
}
