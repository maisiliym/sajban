#![crate_name = "askiom_derive"]
#![crate_type = "proc-macro"]

#[proc_macro_derive(Askiomaiz, attributes(askiom))]
pub fn derive_askiom(input: TokenStream) -> TokenStream { }
