use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

/// Automatically derive Serialize for the given unit struct.
#[proc_macro_derive(Serialize_unit_struct)]
pub fn serialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct.
    let name = &input.ident;
    let name_str = LitStr::new(&name.to_string(), name.span());

    // Construct the impl block.
    let serialize_impl = quote! {
        impl serde::Serialize for #name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serde::Serialize::serialize(#name_str, serializer)
            }
        }
    };

    TokenStream::from(serialize_impl)
}

/// Automatically derive Deserialize for the given unit struct.
#[proc_macro_derive(Deserialize_unit_struct)]
pub fn deserialize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct.
    let name = &input.ident;
    let name_str = LitStr::new(&name.to_string(), name.span());
    let error_msg = LitStr::new(
        &format!("expected unit struct {}", name),
        Span::call_site().into(),
    );

    // Construct the impl block.
    let deserialize_impl = quote! {
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                <String as serde::Deserialize>::deserialize(deserializer)
                    .and_then(|s| {
                        if &s == #name_str {
                            Ok(Self)
                        } else {
                            Err(serde::de::Error::custom(#error_msg))
                        }
                    })
            }
        }
    };

    TokenStream::from(deserialize_impl)
}
