use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Result as SynResult};

use crate::field::generate_field_validation;

pub(crate) fn impl_validate(input: &DeriveInput) -> SynResult<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(_) => {
                return Err(syn::Error::new_spanned(
                    input,
                    "Validate can only be derived for structs with named fields",
                ))
            }
            Fields::Unit => {
                return Err(syn::Error::new_spanned(
                    input,
                    "Validate cannot be derived for unit structs",
                ))
            }
        },
        Data::Enum(_) => {
            return Err(syn::Error::new_spanned(
                input,
                "Validate can only be derived for structs, not enums",
            ))
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                input,
                "Validate can only be derived for structs, not unions",
            ))
        }
    };

    let field_validations: Vec<TokenStream> = fields
        .iter()
        .filter_map(|field| generate_field_validation(field).transpose())
        .collect::<SynResult<Vec<_>>>()?;

    let expanded = quote! {
        impl #impl_generics ::underlay_validation::Validate for #name #ty_generics #where_clause {
            fn validate(&self) -> ::underlay_validation::ValidationResult<()> {
                let mut errors = ::underlay_validation::ValidationError::new();

                #(#field_validations)*

                errors.into_result()
            }
        }
    };

    Ok(expanded)
}
