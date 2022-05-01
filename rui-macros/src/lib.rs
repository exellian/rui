use proc_macro::{Span, TokenStream, TokenTree};
use std::iter;
use quote::{quote, quote_spanned};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    //let main_function = parse_macro_input!(item);

    TokenStream::from(quote! {
        fn main() {

        }
    })
}
