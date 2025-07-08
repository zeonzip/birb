use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, GenericParam, Lifetime, LifetimeParam, parse_macro_input};

#[proc_macro_derive(SerializedErrorResponse)]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let ident = derive_input.ident;
    let mut generics = derive_input.generics;

    let lifetimed_generic = generics.clone();

    let (_, ty_generics, _) = lifetimed_generic.split_for_impl();

    generics.params.insert(
        0,
        GenericParam::Lifetime(LifetimeParam::new(Lifetime::new(
            "'err",
            proc_macro2::Span::call_site(),
        ))),
    );

    let (impl_generics, _, _) = generics.split_for_impl();

    quote! {
        impl #impl_generics From<#ident #ty_generics> for (StatusCode, Json<ErrorResponse<'err>>) {
            fn from(value: #ident #ty_generics) -> Self {
                value.error().response()
            }
        }
    }
    .into()
}
