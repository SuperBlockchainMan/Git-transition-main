use crate::{version::{Version, Versions}, ArgsField};
use darling::{FromMeta, util::parse_attribute_to_meta_list};
use proc_macro2::{TokenStream as TokenStream2, Ident};
use quote::quote;
use syn::{
    visit_mut::{self, VisitMut},
    Fields,
    ItemImpl, ItemStruct, Type, NestedMeta, Field, Visibility, Attribute, ItemEnum,
};
use unsigned_varint::encode::{u64_buffer, self};

pub fn generate_default_enum(visibility: &Visibility, struct_attrs: &Vec<Attribute>, struct_ident: &syn::Ident, structs: &Vec<Ident>) -> TokenStream2 {
    let mut variants = Vec::new();
    for ident in structs {
        let variant = quote! {
            #ident(#ident)
        };
        variants.push(variant);
    }
    let default_enum = quote! {
        #(#struct_attrs)*
        #visibility enum #struct_ident {
            #(#variants),*
        }
    };
    return default_enum;
}

fn filter_variants(enum_version: &mut ItemEnum, version: &Version) {
    enum_version.variants = enum_version.variants.iter().filter_map(|variant| {
        let mut final_variant = Some(variant.clone());
        let final_attrs = variant.attrs.iter().filter_map(|attr| {
            if attr.path.segments.len() > 1 {
                if attr.path.segments[0].ident == "transition" && attr.path.segments[1].ident == "variant" {
                    let attr_args = parse_attribute_to_meta_list(attr).unwrap();
                    let args = ArgsField::from_list(&attr_args.nested.into_iter().collect::<Vec<NestedMeta>>()).unwrap();
                    if !args.versions.0.contains(version) {
                        final_variant = None;
                    }
                    None
                } else {
                    Some(attr.clone())
                }
            } else {
                Some(attr.clone())
            }
        }).collect();
        if let Some(v) = &mut final_variant {
            v.attrs = final_attrs;
        }
        final_variant
    }).collect();
}

fn filter_fields(struct_version: &mut ItemStruct, version: &Version) {
    if let Fields::Named(fields) = &mut struct_version.fields {
        fields.named = fields.named.iter().filter_map(|field: &Field| {
            let mut final_field = Some(field.clone());
            let final_attrs = field.attrs.iter().filter_map(|attr| {
                if attr.path.segments.len() > 1 {
                    if attr.path.segments[0].ident == "transition" && attr.path.segments[1].ident == "field" {
                        let attr_args = parse_attribute_to_meta_list(attr).unwrap();
                        let args = ArgsField::from_list(&attr_args.nested.into_iter().collect::<Vec<NestedMeta>>()).unwrap();
                        if !args.versions.0.contains(version) {
                            final_field = None;
                        }
                        None
                    } else {
                        Some(attr.clone())
                    }
                } else {
                    Some(attr.clone())
                }
            }).collect();
            if let Some(f) = &mut final_field {
                f.attrs = final_attrs;
            }
            final_field
        }).collect();
    }
}

// (Structs, implementations of Versioned trait)
pub fn generate_versioned_struct(input: &ItemStruct, versions: &Versions) -> (Vec<ItemStruct>, Vec<TokenStream2>) {
    let mut structs = Vec::new();
    let mut impls = Vec::new();
    for version in &versions.0 {
        let mut struct_version = input.clone();
        struct_version.ident = version.to_ident(&input.ident);
        filter_fields(&mut struct_version, version);
        let ident = &struct_version.ident;
        let version = version.version;
        let len_version = encode::u64(version, &mut u64_buffer()).len();
        impls.push(quote!(
            impl Versioned for #ident {
                const VERSION: u64 = #version;
                const VERSION_VARINT_SIZE_BYTES: usize = #len_version;
            }
        ));
        structs.push(struct_version);
    }
    return (structs, impls);
}

// (Enum, implementations of Versioned trait)
pub fn generate_versioned_enum(
    input: &ItemEnum, versions: &Versions
) -> (Vec<ItemEnum>, Vec<TokenStream2>) {
    let mut enums = Vec::new();
    let mut impls = Vec::new();
    for version in &versions.0 {
        let mut enum_version = input.clone();
        enum_version.ident = version.to_ident(&input.ident);
        filter_variants(&mut enum_version, version);
        let ident = &enum_version.ident;
        let version = version.version;
        let len_version = encode::u64(version, &mut u64_buffer()).len();
        impls.push(quote!(
            impl Versioned for #ident {
                const VERSION: u64 = #version;
                const VERSION_VARINT_SIZE_BYTES: usize = #len_version;
            }
        ));
        enums.push(enum_version);
    }
    return (enums, impls);
}

// (macro for the type of the struct, macro for the variant of the enum with all types)
pub fn generate_versioned_f_like_macros(
    ident: &Ident,
    versions: &Versions,
) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
    let mut f_like_macros = Vec::new();
    let mut f_like_macros_variant = Vec::new();
    for version in &versions.0 {
        let version_ident = format!("{}", version.version);
        let struct_version = version.to_ident(&ident);
        f_like_macros.push(quote!([#version_ident] => { #struct_version };));
        f_like_macros_variant.push(quote!([#version_ident] => { #ident::#struct_version };));
    }
    return (f_like_macros, f_like_macros_variant);
}

struct ImplVisitor<'a> {
    version: &'a Version,
    struct_ident: &'a Vec<syn::Ident>,
}

impl<'a> VisitMut for ImplVisitor<'a> {
    fn visit_type_mut(&mut self, node: &mut Type) {
        //println!("visit_type_mut: {:?}", node);
        if let Type::Path(path) = node {
            if let Some(ident) = path.path.get_ident() {
                for struct_ident in self.struct_ident {
                    if struct_ident == ident {
                        path.path.segments[0].ident = self.version.to_ident(ident);
                        break;
                    }
                }
            }

        }
        visit_mut::visit_type_mut(self, node);
    }

    fn visit_path_mut(&mut self, i: &mut syn::Path) {
        if let Some(ident) = i.get_ident() {
            for struct_ident in self.struct_ident {
                if struct_ident == ident {
                    i.segments[0].ident = self.version.to_ident(ident);
                    break;
                }
            }
        }
        visit_mut::visit_path_mut(self, i);
    }

    fn visit_expr_type_mut(&mut self, i: &mut syn::ExprType) {
        if let Type::Path(path) = &mut *i.ty {
            if let Some(ident) = path.path.get_ident() {
                for struct_ident in self.struct_ident {
                    if struct_ident == ident {
                        path.path.segments[0].ident = self.version.to_ident(ident);
                        break;
                    }
                }
            }
        }
        visit_mut::visit_expr_type_mut(self, i);
    }

    fn visit_expr_path_mut(&mut self, i: &mut syn::ExprPath) {
        let ident = &i.path.segments[0].ident;
        for struct_ident in self.struct_ident {
            if struct_ident == ident {
                i.path.segments[0].ident = self.version.to_ident(&ident);
                break;
            }
        }
        visit_mut::visit_expr_path_mut(self, i);
    }

    fn visit_expr_struct_mut(&mut self, i: &mut syn::ExprStruct) {
        if let Some(ident) = i.path.get_ident() {
            for struct_ident in self.struct_ident {
                if struct_ident == ident {
                    i.path.segments[0].ident = self.version.to_ident(ident);
                    break;
                }
            }
        }
        visit_mut::visit_expr_struct_mut(self, i);
    }
}

pub fn generate_versioned_impls(
    struct_ident: &Option<Vec<syn::Ident>>,
    input: &ItemImpl,
    versions: &Versions,
) -> Vec<ItemImpl> {
    let mut impls = Vec::new();
    for version in &versions.0 {
        let mut impl_version = input.clone();
        if let Some(struct_ident) = struct_ident {
            let mut visitor = ImplVisitor {
                version,
                struct_ident,
            };
            visitor.visit_item_impl_mut(&mut impl_version);
            impls.push(impl_version);
        } else {
            if let Type::Path(path) = &*input.self_ty {
                if let Some(ident) = path.path.get_ident() {
                    //println!("impl_version: {:?}", impl_version);
                    let mut visitor = ImplVisitor {
                        version,
                        struct_ident: &vec![ident.clone()],
                    };
                    visitor.visit_item_impl_mut(&mut impl_version);
                    impls.push(impl_version);
                }
            }
        }
    }
    return impls;
}