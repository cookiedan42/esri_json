use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ItemFn, Token, Type, parse::Parse, parse::ParseStream, parse_macro_input};

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

struct JsonRoundtripInput {
    ty: Type,
    input: Expr,
}

impl Parse for JsonRoundtripInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let expr: Expr = input.parse()?;
        Ok(JsonRoundtripInput { ty, input: expr })
    }
}

/// Serialize a string, deserialize it, and then compare it to the original string
/// note: all numbers are converted to f64 for comparison
#[proc_macro]
pub fn assert_json_roundtrip(input: TokenStream) -> TokenStream {
    let JsonRoundtripInput {
        ty,
        input: json_input,
    } = parse_macro_input!(input as JsonRoundtripInput);
    let expanded = quote! {
        {

            fn normalize_json(value: serde_json::Value) -> serde_json::Value {
                match value {
                    serde_json::Value::Number(n) => {
                        // Convert all numbers to f64 for comparison
                        let f = n.as_f64().unwrap_or(0.0);
                        serde_json::Value::Number(
                            serde_json::Number::from_f64(f).unwrap_or_else(|| serde_json::Number::from(0))
                        )
                    }
                    serde_json::Value::Array(arr) => {
                        serde_json::Value::Array(arr.into_iter().map(normalize_json).collect())
                    }
                    serde_json::Value::Object(obj) => {
                        serde_json::Value::Object(
                            obj.into_iter().map(|(k, v)| (k, normalize_json(v))).collect()
                        )
                    }
                    other => other,
                }
        }
            let obj = serde_json::from_str::<#ty>(#json_input).unwrap();
            let serialized = serde_json::to_string(&obj).unwrap();

            let original_value = serde_json::from_str::<serde_json::Value>(#json_input).unwrap();
            let roundtrip_value = serde_json::from_str::<serde_json::Value>(&serialized).unwrap();
            assert_eq!(normalize_json(original_value), normalize_json(roundtrip_value));
        }
    };

    expanded.into()
}
