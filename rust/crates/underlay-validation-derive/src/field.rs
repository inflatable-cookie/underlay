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
    let option_inner = option_inner_type(&field.ty);
    let is_option = option_inner.is_some();
    // A blank-string check only makes sense for string payloads. Other `Option<T>`
    // payloads get presence checking alone, which the `None` branch below enforces.
    let option_payload_is_string = option_inner.is_some_and(is_string_like);

    let mut validations = Vec::new();
    let mut is_required = false;

    for attr in &field.attrs {
        if !attr.path().is_ident("validate") {
            continue;
        }

        let parsed = parse_validate_attr(
            attr,
            field_name,
            &field_name_str,
            is_option,
            option_payload_is_string,
        )?;
        if parsed.skip {
            return Ok(None);
        }
        is_required |= parsed.required;
        validations.extend(parsed.validations);
    }

    // `Option<T>` with `required` still needs the `None` branch, even when no
    // value validator applies to the payload type.
    if validations.is_empty() && !(is_option && is_required) {
        return Ok(None);
    }

    let combined = if is_option {
        if is_required {
            quote! {
                if let Some(ref __underlay_val) = self.#field_name {
                    #(#validations)*
                } else {
                    errors.add_field(
                        #field_name_str,
                        ::underlay_validation::FieldError::with_code(
                            "This field is required",
                            "required",
                        ),
                    );
                }
            }
        } else {
            quote! {
                if let Some(ref __underlay_val) = self.#field_name {
                    #(#validations)*
                }
            }
        }
    } else {
        quote! {
            #(#validations)*
        }
    };

    Ok(Some(combined))
}

/// Returns the payload type of `Option<T>`, or `None` for any other type.
fn option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    let mut current = ty;
    while let syn::Type::Group(group) = current {
        current = &group.elem;
    }

    let syn::Type::Path(type_path) = current else {
        return None;
    };
    if type_path.qself.is_some() {
        return None;
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
    if !is_option_path {
        return None;
    }

    let last = segments.last()?;
    let syn::PathArguments::AngleBracketed(args) = &last.arguments else {
        return None;
    };
    if args.args.len() != 1 {
        return None;
    }
    match args.args.first()? {
        syn::GenericArgument::Type(inner) => Some(inner),
        _ => None,
    }
}

/// Whether a type is `String` or `str`, optionally behind references.
///
/// Matching is syntactic: a newtype that only derefs to `str` is not detected and
/// gets presence checking alone.
fn is_string_like(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Group(group) => is_string_like(&group.elem),
        syn::Type::Paren(paren) => is_string_like(&paren.elem),
        syn::Type::Reference(reference) => is_string_like(&reference.elem),
        syn::Type::Path(type_path) if type_path.qself.is_none() => type_path
            .path
            .segments
            .last()
            .is_some_and(|seg| seg.ident == "String" || seg.ident == "str"),
        _ => false,
    }
}
