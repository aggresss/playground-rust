// src/lib.rs (default entry point for proc macros)

extern crate proc_macro; // Apparently needed to be imported like this.

use proc_macro::TokenStream;

#[proc_macro_attribute] // Can now be used as `#[my_attribute]`
pub fn my_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
