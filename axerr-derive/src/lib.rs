use darling::{util, FromDeriveInput, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(axerr), supports(enum_any))]
struct Axerr {
    ident: syn::Ident,
    data: darling::ast::Data<AxerrVariant, util::Ignored>,
}

#[derive(FromVariant, Debug)]
#[darling(attributes(axerr))]
struct AxerrVariant {
    ident: syn::Ident,

    #[darling(default = "default_public_message")]
    public_message: String,
    #[darling(default = "default_status_code")]
    status_code: u16,
}

fn default_status_code() -> u16 {
    500
}

fn default_public_message() -> String {
    "An internal server error occurred".into()
}

#[proc_macro_derive(AxErr, attributes(axerr))]
pub fn axerr_derive(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);

    match axum_derive_inner(parsed_input) {
        Ok(token_stream) => token_stream,
        Err(err) => syn::Error::into_compile_error(err.into()).into(),
    }
}

fn axum_derive_inner(input: DeriveInput) -> Result<TokenStream, darling::Error> {
    let axerr = Axerr::from_derive_input(&input)?;

    let name = axerr.ident;

    let match_branches: proc_macro2::TokenStream = axerr
        .data
        .take_enum()
        .unwrap() // This case is already covered by Axerr::from_derive_input
        .into_iter()
        .map(|variant| {
            let ident = variant.ident;
            let public_message =
                LitStr::new(&variant.public_message, proc_macro2::Span::call_site());

            let status_code = variant.status_code;

            quote! {
                #name::#ident => (axum::http::StatusCode::from_u16(#status_code).unwrap(), #public_message),
            }
        })
        .collect();

    Ok(quote! {
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                axum::response::IntoResponse::into_response(
                    match self {
                        #match_branches
                    }
                )
            }
        }
    }
    .into())
}
