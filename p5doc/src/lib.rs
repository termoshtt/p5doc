//! p5doc proc-macro

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_attribute]
pub fn p5doc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    p5doc2(item.into()).into()
}

fn p5doc2(item: TokenStream2) -> TokenStream2 {
    item
}
