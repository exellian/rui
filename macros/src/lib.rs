use proc_macro::{TokenStream};
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let main_function = parse_macro_input!(stream as );



    TokenStream::from(quote! {
        fn main() {

        }
    })
}
