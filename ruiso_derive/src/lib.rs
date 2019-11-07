#![deny(warnings, missing_docs)]

//! # Ruiso derive
//! 
//! Provides 2 macros for generating features from struct and enums. 
//! 
//! 
//! 


extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, DeriveInput, Ident, Type};

fn set_value_enum(name: &syn::Ident, i: usize, variant: &syn::Variant) -> proc_macro2::TokenStream {
    let v_name = &variant.ident;
    quote! {
        #name::#v_name => slice[#i] = 1.0
    }
}

/*
fn set_value_one_enum(name: &syn::Ident, i: usize, variant: &syn::Variant) -> proc_macro2::TokenStream {
    let v_name = &variant.ident;
    quote! {
        #name::#v_name => slice[0] = #i as f32
    }
}
*/

/// # Enum Featurization
/// Produces a featurizer for your enum that encodes the enum in a one hot manner.
/// This is named ____FeaturizerN where N is the number of attributes in the enum.
/// For example:
/// ```rust
/// ---
/// #[derive(EnumFeature)]
/// pub enum ExampleEnum {
///     Foo,
///     Bar,
///     Kal,
///     Ell,
/// }
/// ```
/// produces ExampleEnumFeaturization4
///
#[proc_macro_derive(EnumFeature, attributes(enum_feature))]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input! {input as DeriveInput};
    let variants = match &input.data {
        syn::Data::Enum(d) => &d.variants,
        _ => panic!("Need a enum"),
    };
    let dim = variants.len();

    let enum_name = &input.ident;
    let featurizer_name = Ident::new(
        &(enum_name.to_string() + &format!("Featurizer{}", dim)),
        Span::call_site(),
    );
    let variant_setters: Vec<proc_macro2::TokenStream> = variants
        .iter()
        .enumerate()
        .map(|(i, v)| set_value_enum(&enum_name, i, v))
        .collect();
    let variant_setters2 = variant_setters.clone();

    let trait_impl = quote! {
        pub struct #featurizer_name{}
        impl Featurizer<#enum_name> for #featurizer_name {
            fn dim() -> usize {#dim}
            fn fill_slice(data:&#enum_name, slice:&mut [f32]) {
                match *data {
                    #(#variant_setters),*
                };
            }
            fn default(_slice: &mut [f32]) {}
        }
        impl Featurizable for #enum_name {
            fn dim() -> usize {#dim}
            fn fill_slice(&self, slice:&mut [f32]) {
                match self {
                    #(#variant_setters2),*
                };
            }
            fn default(_slice: &mut [f32]) {}
        }
    };
    TokenStream::from(trait_impl)
}

fn detect_optional(field: &syn::Field) -> bool {
    let f_type = &field.ty;
    let f_path = match &f_type {
        Type::Path(p) => &p.path,
        _ => panic!("should be a type"),
    };
    (&f_path.segments[0].ident == "Option")
}

fn get_underlying_type_option(f_type: &syn::Type) -> &Type {
    let f_path = match &f_type {
        Type::Path(p) => &p.path,
        _ => panic!("should be a type"),
    };
    if &f_path.segments[0].ident == "Option" {
        let arguments = &f_path.segments[0].arguments;
        let generics = match &arguments {
            syn::PathArguments::AngleBracketed(generic_args) => generic_args,
            _ => panic!("Incorrect arguments in {:?} field", f_type),
        };
        let first_generic = match &generics.args[0] {
            syn::GenericArgument::Type(first) => first,
            _ => panic!("Incorrect arguments in {:?} field", f_type),
        };
        first_generic
    } else {
        f_type
    }
}

fn default_field_handler(field: &syn::Field) -> Option<syn::LitFloat> {
    if let Some(attr) = field.attrs.first() {
        let meta = match attr.parse_meta().unwrap() {
            syn::Meta::List(ml) => match &ml.nested[0] {
                syn::NestedMeta::Meta(m) => m.clone(),
                _ => panic!("meta is not a list"),
            },
            _ => panic!("meta is not a list"),
        };
        if let syn::Meta::NameValue(mv) = meta {
            if mv.path.is_ident("default") {
                if let syn::Lit::Float(v) = mv.lit {
                    Some(v)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn set_basic_field(name:&syn::Ident,i: usize, field: &syn::Field) -> (usize, proc_macro2::TokenStream) {
    let iplus = i + 1;
    let field_name = &field.ident;
    let tokens;
    if detect_optional(field) {
        match default_field_handler(&field) {
            Some(f) => {
                tokens = quote! {
                    if let Some(x) = #name.#field_name {
                        slice[#i] = x as f32;
                    } else {
                        slice[#i] = #f;
                    }
                };
            }
            None => {
                tokens = quote! {
                    if let Some(x) = #name.#field_name {
                        slice[#i] = x as f32;
                    }
                };
            }
        }
    } else {
        tokens = quote! {
            slice[#i] = #name.#field_name as f32;
        };
    }
    (iplus, tokens)
}

fn set_bool_field(name:&syn::Ident,i: usize, field: &syn::Field) -> (usize, proc_macro2::TokenStream) {
    let iplus = i + 1;
    let field_name = &field.ident;
    let tokens;
    if detect_optional(field) {
        match default_field_handler(&field) {
            Some(f) => {
                tokens = quote! {
                    if let Some(x) = #name.#field_name {
                        if x {
                            slice[#i] = 1.0;
                        }
                    } else {
                        slice[#i] = #f;
                    }
                };
            }
            None => {
                tokens = quote! {
                    if let Some(x) = #name.#field_name {
                        if x {
                            slice[#i] = 1.0;
                        }
                    }
                };
            }
        }
    } else {
        tokens = quote! {
            if &#name.#field_name {
                slice[#i] = 1.0;
            }
        };
    }
    (iplus, tokens)
}

fn string_dimension_handler(field: &syn::Field) -> Option<syn::LitInt> {
    if let Some(attr) = field.attrs.first() {
        let meta = match attr.parse_meta().unwrap() {
            syn::Meta::List(ml) => match &ml.nested[0] {
                syn::NestedMeta::Meta(m) => m.clone(),
                _ => panic!("meta is not a list"),
            },
            _ => panic!("meta is not a list"),
        };
        if let syn::Meta::NameValue(mv) = meta {
            if mv.path.is_ident("dim") {
                if let syn::Lit::Int(v) = mv.lit {
                    Some(v)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn set_string_field(name:&syn::Ident,i: usize, field: &syn::Field) -> (usize, proc_macro2::TokenStream) {
    let dim: usize = match string_dimension_handler(field) {
        Some(d) => d.base10_parse().unwrap(),
        None => 37,
    };
    let iplus = i + dim;
    let field_name = &field.ident;

    let tokens;
    if detect_optional(field) {
        tokens = quote! {
            if let Some(x) = &#name.#field_name {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                x.hash(&mut hasher);
                let result = (hasher.finish() as usize) % #dim;
                slice[(#i)+result] += 1.0;
            }
        };
    } else {
        tokens = quote! {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            #name.#field_name.hash(&mut hasher);
            let result = (hasher.finish() as usize) % #dim;
            slice[(#i)+result] += 1.0;
        };
    }
    (iplus, tokens)
}

fn set_vec_field(name:&syn::Ident,i: usize, field: &syn::Field) -> (usize, proc_macro2::TokenStream) {
    let dim: usize = match string_dimension_handler(field) {
        Some(d) => d.base10_parse().unwrap(),
        None => 37,
    };
    let iplus = i + dim;
    let field_name = &field.ident;
    let tokens;

    if let Type::Path(pat) = &get_underlying_type_option(&field.ty) {
        if let syn::PathArguments::AngleBracketed(pat) =
            &pat.path.segments.last().unwrap().arguments
        {
            if let syn::GenericArgument::Type(pat) = pat.args.last().unwrap() {
                if let syn::Type::Path(pat) = pat {
                    if pat.path.is_ident("String") {
                        if detect_optional(field) {
                            tokens = quote! {
                                if let Some(x) = &#name.#field_name {
                                    for s in x {
                                        let mut hasher = std::collections::hash_map::DefaultHasher::new();
                                        s.hash(&mut hasher);
                                        let result = (hasher.finish() as usize) % #dim;
                                        slice[(#i)+result] += 1.0;
                                    }
                                }
                            };
                        } else {
                            tokens = quote! {
                                for s in &#name.#field_name {
                                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                                    s.hash(&mut hasher);
                                    let result = (hasher.finish() as usize) % #dim;
                                    slice[(#i)+result] += 1.0;
                                }
                            };
                        }
                    } else {
                        panic!("{:?}",);
                    }
                } else {
                    panic!("{:?}",);
                }
            } else {
                panic!("{:?}",);
            }
        } else {
            panic!("{:?}",);
        }
    } else {
        panic!("{:?}",);
    }
    (iplus, tokens)
}

fn set_custom_field(
    name:&syn::Ident,
    i: usize,
    field: &syn::Field,
    featurizer: syn::Ident,
    dimension: u16,
) -> (usize, proc_macro2::TokenStream) {
    let iplus = i + dimension as usize;
    let field_name = &field.ident;
    let tokens;
    if detect_optional(field) {
        tokens = quote! {
            if let Some(x) = &#name.#field_name {
                #featurizer::fill_slice(x,&mut slice[#i..#iplus]);
            } else {
                #featurizer::default(&mut slice[#i..#iplus]);
            }
        };
    } else {
        tokens = quote! {
            #featurizer::fill_slice(&#name.#field_name,&mut slice[#i..#iplus]);
        };
    }
    (iplus, tokens)
}

fn custom_featurizer_handler(field: &syn::Field) -> Option<(syn::Ident, u16)> {
    if let Some(attr) = field.attrs.first() {
        let meta = match attr.parse_meta().unwrap() {
            syn::Meta::List(ml) => match &ml.nested[0] {
                syn::NestedMeta::Meta(m) => m.clone(),
                _ => panic!("meta is not a list"),
            },
            _ => panic!("meta is not a list"),
        };
        if let syn::Meta::NameValue(mv) = meta {
            if mv.path.is_ident("featurizer") {
                if let syn::Lit::Str(v) = mv.lit {
                    let re = Regex::new(r"[[:alpha:]]*([0-9]*)").unwrap();
                    let dimension: u16 = match re.captures(&v.value()).unwrap().get(1) {
                        Some(m) => m.as_str().parse().unwrap(),
                        None => panic!("Featurizers need to end in their dimension."),
                    };
                    let ident = Ident::new(&v.value(), Span::call_site());
                    Some((ident, dimension))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn set_value_field(name:&syn::Ident,i: usize, field: &syn::Field) -> (usize, proc_macro2::TokenStream) {
    match custom_featurizer_handler(field) {
        Some((custom, len)) => set_custom_field(name,i, field, custom, len),
        None => {
            if let Type::Path(pat) = &get_underlying_type_option(&field.ty) {
                match &pat.path.segments.last().unwrap().ident.to_string().as_str() {
                    &"f32" => set_basic_field(name,i, field),
                    &"f64" => set_basic_field(name,i, field),
                    &"u8" => set_basic_field(name,i, field),
                    &"u16" => set_basic_field(name,i, field),
                    &"u32" => set_basic_field(name,i, field),
                    &"u64" => set_basic_field(name,i, field),
                    &"i8" => set_basic_field(name,i, field),
                    &"i16" => set_basic_field(name,i, field),
                    &"i32" => set_basic_field(name,i, field),
                    &"i64" => set_basic_field(name,i, field),
                    &"usize" => set_basic_field(name,i, field),
                    &"bool" => set_bool_field(name,i, field),
                    &"String" => set_string_field(name,i, field),
                    &"Vec" => set_vec_field(name,i, field),
                    _ => panic!("This field should have a custom featurizer provided"),
                }
            } else {
                panic!("{:?}",);
            }
        }
    }
}

fn detect_off(field: &syn::Field) -> bool {
    if let Some(attr) = field.attrs.first() {
        let meta = match attr.parse_meta().unwrap() {
            syn::Meta::List(ml) => match &ml.nested[0] {
                syn::NestedMeta::Meta(m) => m.clone(),
                _ => panic!("meta is not a list"),
            },
            _ => panic!("meta is not a list"),
        };
        if let syn::Meta::Path(pat) = meta {
            if pat.is_ident("off") {
                return true;
            }
        }
    }
    false
}

/// # Struct Featurization
/// Gives the struct a  
/// For example:
/// ```rust
/// ---
/// #[derive(StructFeature)]
/// pub struct TestStruct {
///     foo: u32,
///     #[struct_feature(off)]
///     kal: String,
///     #[struct_feature(default = 5.0)]
///     bar: Option<f32>,
///     #[struct_feature(featurizer = "ExampleEnumFeaturizer4")]
///     ell: ExampleEnum,
///     #[struct_feature(dim = 21)]
///     kan: String,
/// }
/// ```
/// produces TestStructFeaturization27 and enables the trait Featurizable for your struct. 
/// For nesting use the featurizer decoration with the name of the featurizer you want to use.
/// For strings we can specify the dimension of the hashing trick we want to use.
/// We can also turn off fields we don't want to include.
/// For single value fields (u8,f32,i64, etc..) we can give a default value if they are optional.
///
#[proc_macro_derive(StructFeature, attributes(struct_feature))]
pub fn derive_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input! {input as DeriveInput};

    let struct_name = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(d) => &d.fields,
        _ => panic!("Need a Struct"),
    };
    let mut i = 0;
    let name = syn::Ident::new("self",Span::call_site());
    let data = syn::Ident::new("data",Span::call_site());
    let mut self_field_setters: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut name_field_setters: Vec<proc_macro2::TokenStream> = Vec::new();
    for f in fields {
        if !detect_off(f) {
            let (_iplus, k_name) = set_value_field(&name,i, f);
            self_field_setters.push(k_name);
            let (iplus, k_data) = set_value_field(&data,i, f);
            name_field_setters.push(k_data);
            i = iplus;
        }
    }
    let dim = i;

    let featurizer_name = Ident::new(
        &(struct_name.to_string() + &format!("Featurizer{}", dim)),
        Span::call_site(),
    );

    let trait_impl = quote! {
        impl Featurizable for #struct_name {
            fn dim() -> usize {#dim}
            fn fill_slice(&self, slice:&mut [f32]) {
                #(#self_field_setters);*;
            }
            fn default(_slice: &mut [f32]) {}
        }

        pub struct #featurizer_name{}
        impl Featurizer<#struct_name> for #featurizer_name {
            fn dim() -> usize {#dim}
            fn fill_slice(data:&#struct_name, slice:&mut [f32]) {
                #(#name_field_setters);*;
            }
            fn default(_slice: &mut [f32]) {}
        }
    };
    TokenStream::from(trait_impl)
}
