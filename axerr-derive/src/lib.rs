use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(axerr), supports(enum_any))]
struct Axerr {}

#[derive(Default, FromMeta)]
struct AxerrField {
    #[darling(default = "default_message")]
    message: String,
    #[darling(default = "default_status_code")]
    status_code: u16,
}

fn default_status_code() -> u16 {
    500
}

fn default_message() -> String {
    "An internal server error occurred".into()
}

#[proc_macro_derive(AxErr)]
pub fn axerr_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let enum_data = match input.data {
        syn::Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(
                name.clone(),
                "AxErr can only be derived for enumerations.",
            )
            .into_compile_error()
            .into()
        }
    };

    for variant in enum_data.variants.into_iter() {
        let variant_name = variant.ident;

        let maybe_attribute = variant
            .attrs
            .into_iter()
            .find(|attr| attr.path().is_ident("axerr"));

        if let Some(attribute) = maybe_attribute {
            let meta = attribute.parse_nested_meta(|meta| {});
        } else {
            // Fallback on default.
        }
    }

    quote! {
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                match self {

                }
            }
        }
    }
    .into()
}
