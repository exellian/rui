use proc_macro::TokenStream;
use std::collections::HashMap;

use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::parse_quote::ParseQuote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, ItemFn, ReturnType, Signature, Token};

struct NamedArg {
    name: Ident,
    val: Ident,
}
impl Parse for NamedArg {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        // parses named arguments e.g: a = b, c=d, f=DirectX12
        let parts = Punctuated::<Ident, Token![=]>::parse_terminated(input)?;
        if parts.len() != 2 {
            panic!("Invalid argument expression!");
        }
        let name = parts.first().unwrap().clone();
        let val = parts.last().unwrap().clone();
        Ok(NamedArg { name, val })
    }
}

struct NamedArgs {
    args: HashMap<Ident, Ident>,
}

impl Parse for NamedArgs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        // parses a = b, c=d, f=DirectX12
        let mut args = HashMap::new();
        //let lol =  Punctuated::<Ident, Token![=]>::parse(input)?;
        let input = Punctuated::<NamedArg, Token![,]>::parse_terminated(input)?;
        for item in input {
            if args.contains_key(&item.name) {
                panic!("Duplicate argument `{}` !", item.name.to_string())
            }
            args.insert(item.name, item.val);
        }
        Ok(NamedArgs { args })
    }
}

fn has_generics(sig: &Signature) -> bool {
    return if let Some(_) = sig.generics.lt_token {
        true
    } else if let Some(_) = sig.generics.gt_token {
        true
    } else if let Some(_) = sig.generics.where_clause {
        true
    } else {
        false
    };
}

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let main = parse_macro_input!(item as ItemFn);
    let args = parse_macro_input!(args as NamedArgs);
    if main.sig.asyncness.is_none() {
        panic!("Rui main must be an async function!");
    }
    if let ReturnType::Type(_, _) = main.sig.output {
        panic!("Rui main cannot have a return type!");
    }
    if let Some(_) = main.sig.abi {
        panic!("Rui main should not have abi!");
    }
    if let Some(_) = main.sig.constness {
        panic!("Rui main should not be const!");
    }
    if has_generics(&main.sig) {
        panic!("Rui main should not have generics!");
    }
    if main.sig.ident != "main" {
        panic!("Rui main must have the name `main` !");
    }
    if !main.sig.inputs.is_empty() {
        panic!("Rui main cannot have arguments!");
    }
    if let Some(_) = main.sig.unsafety {
        panic!("Rui main cannot be unsafe!");
    }
    if let Some(_) = main.sig.variadic {
        panic!("Rui main cannot be variadic!");
    }
    // TODO interpret named args

    let body = main.block.into_token_stream();

    TokenStream::from(quote! {
        fn main() {
            rui::reactor::Reactor::init();
            let instance = rui::instance::Instance::default();

            async fn _main() {
                #body
            }
            instance.run(_main())
        }
    })
}
