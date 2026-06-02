use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Token, Type, parse::Parse, parse::ParseStream, parse_macro_input};

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
pub(crate) fn assert_json_roundtrip(input: TokenStream) -> TokenStream {
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
