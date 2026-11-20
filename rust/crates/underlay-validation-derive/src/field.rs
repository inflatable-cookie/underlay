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
    let is_option = is_option_type(&field.ty);

    let mut validations = Vec::new();

    for attr in &field.attrs {
        if !attr.path().is_ident("validate") {
            continue;
        }

        let (parsed, skip) = parse_validate_attr(attr, field_name, &field_name_str, is_option)?;
        if skip {
            return Ok(None);
        }
        validations.extend(parsed);
    }

    if validations.is_empty() {
        return Ok(None);
    }

    let combined = if is_option {
        quote! {
            if let Some(ref __underlay_val) = self.#field_name {
                #(#validations)*
            }
        }
    } else {
        quote! {
            #(#validations)*
        }
    };

    Ok(Some(combined))
}

fn is_option_type(ty: &syn::Type) -> bool {
    let mut current = ty;
    while let syn::Type::Group(group) = current {
        current = &group.elem;
    }

    let syn::Type::Path(type_path) = current else {
        return false;
    };
    if type_path.qself.is_some() {
        return false;
    }

    let segments = &type_path.path.segments;
    let is_option_path = match segments.len() {
        1 => segments[0].ident == "Option",
        3 => {
            (segments[0].ident == "std" || segments[0].ident == "core")
                && segments[1].ident == "option"
                && segments[2].ident == "Option"
        }
        _ => false,
    };

    is_option_path
        && matches!(
            segments.last().map(|seg| &seg.arguments),
            Some(syn::PathArguments::AngleBracketed(args)) if args.args.len() == 1
        )
}
