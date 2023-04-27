extern crate rand;

use std::panic;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum,DeriveInput};
use hashbrown::HashMap;


#[proc_macro_derive(CylinerBuildU8)]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    // Get Inital TokenStream
    let description = match data {
        syn::Data::Struct(_) => panic!("Can not run on Structs!"),
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let vs  = variants.iter().map(|v| &v.ident);
            let x = quote! {#(#vs),*};
            format!("{}",x)
        }
        syn::Data::Union(_) => panic!("Can not run on Unions!"),
    };
    // Generate hash map
    let desv = description.split(",").collect::<Vec<&str>>();
    let mut value_adds = format!("let mut r_hash: HashMap<&{}, u8>  = HashMap::new();",ident);
    let mut i: u8 = 0;
    for y in desv {
        let r_key = format!("&{}::{}",ident,y.trim());
        value_adds = format!("{}\nr_hash.insert({},{});",value_adds,r_key,i);
        i += 1;
    }
    // Output hashmap
    let sv: proc_macro2::TokenStream = value_adds.parse().unwrap();
    let output = quote! {
        impl #ident {
            fn describe() -> HashMap<&'static #ident, u8>  {
                #sv
                return r_hash
            }
        }
    };

    output.into()
}
