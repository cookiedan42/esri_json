mod json_roundtrip;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ItemImpl, Type, parse_macro_input};

/// Serialize a string, deserialize it, and then compare it to the original string
/// note: all numbers are converted to f64 for comparison
#[proc_macro]
pub fn assert_json_roundtrip(input: TokenStream) -> TokenStream {
    json_roundtrip::assert_json_roundtrip(input)
}

#[proc_macro_attribute]
pub fn test_all_coord_types(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    let existing_predicates = input
        .sig
        .generics
        .where_clause
        .as_ref()
        .map(|w| &w.predicates);

    let expanded = quote! {
        #[rstest::rstest]
        #[case::xy_f32(std::marker::PhantomData::<CoordXy<f32>>)]
        #[case::xy_f64(std::marker::PhantomData::<CoordXy<f64>>)]
        #[case::xyz_f32(std::marker::PhantomData::<CoordXyz<f32>>)]
        #[case::xyz_f64(std::marker::PhantomData::<CoordXyz<f64>>)]
        #[case::xym_f32(std::marker::PhantomData::<CoordXym<f32>>)]
        #[case::xym_f64(std::marker::PhantomData::<CoordXym<f64>>)]
        #[case::xyzm_f32(std::marker::PhantomData::<CoordXyzm<f32>>)]
        #[case::xyzm_f64(std::marker::PhantomData::<CoordXyzm<f64>>)]
        fn #fn_name<C>(#[case] _phantom: std::marker::PhantomData<C>)
        where
            C: Coord + serde::Serialize + serde::de::DeserializeOwned + std::cmp::PartialEq,
            C::T: CoordNumber + From<f32> + serde::Serialize + serde::de::DeserializeOwned,
            geo_types::Coord<C::T>: From<C>,
            #existing_predicates
        #fn_block
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn test_all_number_types(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    let existing_predicates = input
        .sig
        .generics
        .where_clause
        .as_ref()
        .map(|w| &w.predicates);

    let expanded = quote! {
        #[rstest::rstest]
        #[case::f32(std::marker::PhantomData::<f32>)]
        #[case::f64(std::marker::PhantomData::<f64>)]
        fn #fn_name<T>(#[case] _phantom: std::marker::PhantomData<T>)
        where
            T: CoordNumber + From<f32> + serde::Serialize + serde::de::DeserializeOwned,
            #existing_predicates
        #fn_block
    };
    expanded.into()
}

/// Generates `From<T>` that delegates to `From<&T>` via `(&val).into()`
///
/// Use for non-Copy types or complex conversions.
#[proc_macro_attribute]
pub fn from_ref_delegate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let (inner_type, target_type, generics, where_clause) = extract_from_impl(&input);

    let expanded = quote! {
        #input

        impl #generics From<#inner_type> for #target_type
        #where_clause
        {
            fn from(val: #inner_type) -> Self {
                (&val).into()
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generates `From<T>` with the same body as `From<&T>` (copies the implementation)
///
/// Use for Copy types where delegation adds unnecessary indirection.
#[proc_macro_attribute]
pub fn from_ref_copy(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let (inner_type, target_type, generics, where_clause) = extract_from_impl(&input);

    // Extract the function body from the From<&T> impl
    let from_fn = input
        .items
        .iter()
        .find_map(|item| {
            if let syn::ImplItem::Fn(method) = item {
                if method.sig.ident == "from" {
                    return Some(method);
                }
            }
            None
        })
        .expect("Expected a `from` method");

    let body = &from_fn.block;

    let expanded = quote! {
        #input

        impl #generics From<#inner_type> for #target_type
        #where_clause
        {
            fn from(val: #inner_type) -> Self
            #body
        }
    };

    TokenStream::from(expanded)
}

fn extract_from_impl(
    input: &ItemImpl,
) -> (
    &syn::Type,
    &syn::Type,
    &syn::Generics,
    &Option<syn::WhereClause>,
) {
    let trait_path = &input.trait_.as_ref().expect("Expected a trait impl").1;
    let from_segment = trait_path.segments.last().expect("Expected From trait");

    let ref_type = match &from_segment.arguments {
        syn::PathArguments::AngleBracketed(args) => {
            args.args.first().expect("Expected type argument")
        }
        _ => panic!("Expected angle bracketed arguments"),
    };

    let inner_type = match ref_type {
        syn::GenericArgument::Type(Type::Reference(r)) => &*r.elem,
        _ => panic!("Expected reference type in From<&T>"),
    };

    let target_type = &*input.self_ty;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;

    (inner_type, target_type, generics, where_clause)
}
