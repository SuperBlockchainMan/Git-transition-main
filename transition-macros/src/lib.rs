use generation::{generate_versioned_impls, generate_versioned_f_like_macros, generate_default_enum, generate_versioned_enum};
use proc_macro::{TokenStream};
use darling::FromMeta;
use syn::{parse_macro_input, AttributeArgs, ItemStruct, ItemImpl, ItemEnum, NestedMeta, Lit};
use version::Versions;

use crate::generation::generate_versioned_struct;

mod generation;
mod version;

#[derive(Debug, FromMeta)]
struct ArgsStruct {
    versions: Versions
}

#[derive(Debug, FromMeta)]
struct ArgsField {
    versions: Versions
}

#[derive(Debug)]
struct StructuresToReplace(pub Vec<syn::Ident>);

impl FromMeta for StructuresToReplace {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut structures = Vec::new();
        for item in items {
            match item {
                NestedMeta::Lit(Lit::Str(str)) => {
                    structures.push(syn::Ident::from_string(&str.value())?);
                }
                _ => {}
            }
        }
        Ok(StructuresToReplace(structures))
    }
}

#[derive(Debug, FromMeta)]
struct ArgsImpl {
    versions: Versions,
    structures: Option<StructuresToReplace>
}

#[proc_macro_attribute]
pub fn versioned_enum(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let mut args = ArgsStruct::from_list(&attr_args).unwrap();
    args.versions.0.sort();
    let mut enum_versioned = parse_macro_input!(input as ItemEnum);
    enum_versioned.attrs.retain(|attr| !attr.path.is_ident("transition"));
    let (enums, impls) = generate_versioned_enum(&enum_versioned, &args.versions);
    let default_enum = generate_default_enum(&enum_versioned.vis, &enum_versioned.attrs, &enum_versioned.ident, &enums.iter().map(|e| e.ident.clone()).collect());
    let enum_name = enum_versioned.ident.clone();
    let (f_like_macros, f_like_macros_variant) = generate_versioned_f_like_macros(&enum_name, &args.versions);
    let macro_variant_ident = syn::Ident::new(&format!("{}Variant", enum_name), enum_name.span());
    TokenStream::from(quote::quote! {
        #default_enum

        macro_rules! #enum_name {
            #(#f_like_macros)*
        }

        macro_rules! #macro_variant_ident {
            #(#f_like_macros_variant)*
        }

        #(#enums)*

        #(#impls)*
    })
}

#[proc_macro_attribute]
pub fn versioned(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let mut args = ArgsStruct::from_list(&attr_args).unwrap();
    args.versions.0.sort();
    let mut struct_versioned = parse_macro_input!(input as ItemStruct);
    struct_versioned.attrs.retain(|attr| !attr.path.is_ident("transition"));
    let (structs, impls) = generate_versioned_struct(&struct_versioned, &args.versions);
    let default_enum = generate_default_enum(&struct_versioned.vis, &struct_versioned.attrs, &struct_versioned.ident, &structs.iter().map(|s| s.ident.clone()).collect());
    let struct_name = struct_versioned.ident.clone();
    let (f_like_macros, f_like_macros_variant) = generate_versioned_f_like_macros(&struct_name, &args.versions);
    let macro_variant_ident = syn::Ident::new(&format!("{}Variant", struct_name), struct_name.span());
    TokenStream::from(quote::quote! {
        #default_enum

        macro_rules! #struct_name {
            #(#f_like_macros)*
        }

        macro_rules! #macro_variant_ident {
            #(#f_like_macros_variant)*
        }

        #(#structs)*

        #(#impls)*
    })
}

#[proc_macro_attribute]
pub fn impl_version(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let args = ArgsImpl::from_list(&attr_args).unwrap();
    let impl_versioned = parse_macro_input!(input as ItemImpl);
    let impls = generate_versioned_impls(&args.structures.map(|x| x.0), &impl_versioned, &args.versions);
    TokenStream::from(quote::quote! {
        #(#impls)*
    })
}

// Placeholder for field
#[proc_macro_attribute]
pub fn field(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}