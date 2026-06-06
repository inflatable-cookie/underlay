use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Result as SynResult};

use crate::rules::parse_validate_attr;

pub(crate) fn generate_field_validation(field: &Field) -> SynResult<Option<TokenStream>> {
    let field_name = field
        .ident
        .as_ref()
        .ok_or_else(|| syn::Error::new_spanned(field, "Expected named field"))?;

    let field_name_str = field_name.to_string();

    let mut validations = Vec::new();

    for attr in &field.attrs {
        if !attr.path().is_ident("validate") {
            continue;
        }

        let parsed = parse_validate_attr(attr, field_name, &field_name_str)?;
        validations.extend(parsed);
    }

    if validations.is_empty() {
        return Ok(None);
    }

    let combined = quote! {
        #(#validations)*
    };

    Ok(Some(combined))
}
