//! Copyright (c) 2022 Chris Riches
//! (Licensed under MIT or Apache 2.0)

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, LitStr};

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

    // We have to squirt a new Visitor struct into scope; pick a name that definitely
    // won't collide with anything.
    let visitor = Ident::new(
        &format!("SerdeUnitStructDerive{}Visitor", name),
        Span::call_site().into(),
    );

    // Construct the impl block.
    let deserialize_impl = quote! {
        struct #visitor;

        impl<'de> serde::de::Visitor<'de> for #visitor {
            type Value = #name;

            fn expecting(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.write_str(#error_msg)
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                if value == #name_str {
                    Ok(#name)
                } else {
                    Err(E::custom(#error_msg))
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_str(#visitor)
            }
        }
    };

    TokenStream::from(deserialize_impl)
}
