#![forbid(unsafe_code)]
//! Cylinder is an embedded rust library for type-safe inter MCU/MPU communication that supports #![no_std]
//! I created it because handling complex inter-MCU communication without type-safety is extremely difficult, but with
//! Cylinder you don't have to worry about defining unique u8 values for commands, conflicts,
//! and other issues with non type-safe communication.
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum,DeriveInput};
use proc_macro_error::{abort,proc_macro_error};


/// Maps variants in an enum to unique u8 values accessible via an implementation. Which can be used like this:
/// ```ignore
/// use cylinder::CylinerBuildU8;
///
/// #[derive(CylinerBuildU8)]
/// enum Test {
///     MyEnum1,
///     MyEnum2,
///     MyEnum3,
///     MyEnum4
/// }
/// let val : u8 = Test::get_u8(Test::MyEnum2);
/// ```
#[proc_macro_derive(CylinerBuildU8)]
#[proc_macro_error]
pub fn _generate_cylinder_impl(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    // Get Initial TokenStream
    let description = match data {
        syn::Data::Struct(_) => abort!("x","Can not run on Structs! Cylinder only works on enums"),
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let vs  = variants.iter().map(|v| &v.ident);
            let x = quote! {#(#vs),*};
            format!("{}",x)
        }
        syn::Data::Union(_) => abort!("x","Can not run on Unions! Cylinder only works on enums"),
    };
    // Generate u8 mapping
    let enum_variants = description.split(',').collect::<Vec<&str>>();
    let mut impl_vals = String::from("");
    for (i,y) in enum_variants.into_iter().enumerate() {
        impl_vals = format!("{}\n{}::{} => {},", impl_vals, ident, y.trim(), i);
    }
    // Output hashmap
    let parsed_impl_vals: proc_macro2::TokenStream = impl_vals.parse().unwrap();
    let output = quote! {
        impl #ident {
            fn get_u8(self) -> u8 {
                match self {
                  #parsed_impl_vals
                }
            }
        }
    };

    output.into()
}