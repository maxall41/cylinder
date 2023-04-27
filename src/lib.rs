//! Cylinder is an embedded rust library for type-safe inter MCU/MPU communication that supports #![no_std]
//! I created it because handling complex inter-MCU communication without type-safety is extremely difficult, but with
//! Cylinder you don't have to worry about defining unique u8 values for commands, conflicts,
//! and other issues with non type-safe communication.
extern crate rand;

use std::panic;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum,DeriveInput};
use hashbrown::HashMap;

/// Maps variants in enum to u8 values acsessible via a hashbrown hashmap
/// the hashmap can be used like this:
/// ```ignore
/// use cylinder::CylinerBuildU8;
///
/// #[derive(CylinerBuildU8)]
/// #[derive(Eq, Hash, PartialEq,Debug)]
/// enum Test {
///     MyEnum1,
///     MyEnum2,
///     MyEnum3,
///     MyEnum4
/// }
/// let hash : HashMap<&Test, u8> = Test::describe();
/// let val : u8 = hash.get(&Test::MyEnum1);
/// ```
/// Due note that when hashbrown is used in a `#![no_std]` enviroment you need to define a global allocator for the alloc crate such as `embedded-alloc`
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
