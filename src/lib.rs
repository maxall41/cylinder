extern crate rand;

use std::panic;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed};
use rand::Rng;
use hashbrown::HashMap;


#[proc_macro_derive(CylinerBuildU8)]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    // Get Inital TokenStream
    let description = match data {
        syn::Data::Struct(s) => panic!("Can not run on Structs!"),
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let vs  = variants.iter().map(|v| &v.ident);
            let x = quote! {#(#vs),*};
            format!("{}",x) // Almost certainly a better way todo this
        }
        syn::Data::Union(DataUnion) => panic!("Can not run on Unions!"),
    };
    // Generate hash map
    let desv = description.split(",").collect::<Vec<&str>>();
    let mut hash: HashMap<&str, u8>  = HashMap::new();
    let mut i: u8 = 0;
    for y in desv {
        hash.insert(
            y.trim(),
            i,
        );
        i += 1;
    }
    // Reconstruct hash map
    let mut value_adds = String::from("let mut r_hash: HashMap<&str, u8>  = HashMap::new();");
    for (key, value) in &hash {
        value_adds = format!("{}\nr_hash.insert(\"{}\",{});",value_adds,key.trim(),value);
    }
    let sv: proc_macro2::TokenStream = value_adds.parse().unwrap();
    // Output hashmap
    let output = quote! {
        impl #ident {
            fn describe() -> HashMap<&'static str, u8>  {
                #sv
                return r_hash
            }
        }
    };

    output.into()
}

#[proc_macro_derive(CylinerBuildU16)]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    // Get Inital TokenStream
    let description = match data {
        syn::Data::Struct(s) => panic!("Can not run on Structs!"),
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let vs  = variants.iter().map(|v| &v.ident);
            let x = quote! {#(#vs),*};
            format!("{}",x) // Almost certainly a better way todo this
        }
        syn::Data::Union(DataUnion) => panic!("Can not run on Unions!"),
    };
    // Generate hash map
    let desv = description.split(",").collect::<Vec<&str>>();
    let mut hash: HashMap<&str, u16>  = HashMap::new();
    let mut i: u16 = 0;
    for y in desv {
        hash.insert(
            y.trim(),
            i,
        );
        i += 1;
    }
    // Reconstruct hash map
    let mut value_adds = String::from("let mut r_hash: HashMap<&str, u16>  = HashMap::new();");
    for (key, value) in &hash {
        value_adds = format!("{}\nr_hash.insert(\"{}\",{});",value_adds,key.trim(),value);
    }
    let sv: proc_macro2::TokenStream = value_adds.parse().unwrap();
    // Output hashmap
    let output = quote! {
        impl #ident {
            fn describe() -> HashMap<&'static str, u8>  {
                #sv
                return r_hash
            }
        }
    };

    output.into()
}