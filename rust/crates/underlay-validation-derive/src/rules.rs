use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, Ident, Result as SynResult};

pub(crate) struct ParsedValidateAttr {
    pub(crate) validations: Vec<TokenStream>,
    pub(crate) skip: bool,
    pub(crate) required: bool,
}

pub(crate) fn parse_validate_attr(
    attr: &Attribute,
    field_name: &Ident,
    field_name_str: &str,
    is_option: bool,
    option_payload_is_string: bool,
) -> SynResult<ParsedValidateAttr> {
    let mut validations = Vec::new();
    let mut skip = false;
    let mut required = false;

    let ref_expr = if is_option {
        quote! { __underlay_val }
    } else {
        quote! { &self.#field_name }
    };
    let val_expr = if is_option {
        quote! { *__underlay_val }
    } else {
        quote! { self.#field_name }
    };

    attr.parse_nested_meta(|meta| {
        let path = &meta.path;

        // Simple validators (no arguments)
        if path.is_ident("skip") {
            skip = true;
            return Ok(());
        }

        if path.is_ident("email") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::email(#ref_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("url") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::url(#ref_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("uuid") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::uuid(#ref_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("required") {
            required = true;
            // `Option<T>` presence is enforced by the `None` branch in `field.rs`.
            // The blank-string check only applies to string payloads.
            if !is_option || option_payload_is_string {
                validations.push(quote! {
                    if let Err(e) = ::underlay_validation::validators::required(#ref_expr) {
                        errors.add_field(#field_name_str, e);
                    }
                });
            }
            return Ok(());
        }

        if path.is_ident("alphanumeric") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::alphanumeric(#ref_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("username") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::username(#ref_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("slug") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::slug(#ref_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("positive") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::positive(#val_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("non_negative") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::non_negative(#val_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("not_empty") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::not_empty(#ref_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("nested") {
            validations.push(quote! {
                if let Err(nested_errors) = ::underlay_validation::Validate::validate(#ref_expr) {
                    errors.merge_nested(#field_name_str, nested_errors);
                }
            });
            return Ok(());
        }

        // Validators with arguments
        if path.is_ident("length") {
            let mut min_val: Option<TokenStream> = None;
            let mut max_val: Option<TokenStream> = None;

            meta.parse_nested_meta(|inner| {
                if inner.path.is_ident("min") {
                    let value: syn::LitInt = inner.value()?.parse()?;
                    min_val = Some(quote! { Some(#value) });
                } else if inner.path.is_ident("max") {
                    let value: syn::LitInt = inner.value()?.parse()?;
                    max_val = Some(quote! { Some(#value) });
                } else {
                    return Err(inner.error("expected `min` or `max`"));
                }
                Ok(())
            })?;

            let min_expr = min_val.unwrap_or_else(|| quote! { None });
            let max_expr = max_val.unwrap_or_else(|| quote! { None });

            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::length(#ref_expr, #min_expr, #max_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("range") {
            let mut min_val: Option<TokenStream> = None;
            let mut max_val: Option<TokenStream> = None;

            meta.parse_nested_meta(|inner| {
                if inner.path.is_ident("min") {
                    let value: Expr = inner.value()?.parse()?;
                    min_val = Some(quote! { Some(#value) });
                } else if inner.path.is_ident("max") {
                    let value: Expr = inner.value()?.parse()?;
                    max_val = Some(quote! { Some(#value) });
                } else {
                    return Err(inner.error("expected `min` or `max`"));
                }
                Ok(())
            })?;

            let min_expr = min_val.unwrap_or_else(|| quote! { None });
            let max_expr = max_val.unwrap_or_else(|| quote! { None });

            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::range(#val_expr, #min_expr, #max_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("collection_length") {
            let mut min_val: Option<TokenStream> = None;
            let mut max_val: Option<TokenStream> = None;

            meta.parse_nested_meta(|inner| {
                if inner.path.is_ident("min") {
                    let value: syn::LitInt = inner.value()?.parse()?;
                    min_val = Some(quote! { Some(#value) });
                } else if inner.path.is_ident("max") {
                    let value: syn::LitInt = inner.value()?.parse()?;
                    max_val = Some(quote! { Some(#value) });
                } else {
                    return Err(inner.error("expected `min` or `max`"));
                }
                Ok(())
            })?;

            let min_expr = min_val.unwrap_or_else(|| quote! { None });
            let max_expr = max_val.unwrap_or_else(|| quote! { None });

            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::collection_length(#ref_expr, #min_expr, #max_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        // Name-value validators
        if path.is_ident("pattern") {
            let value: syn::LitStr = meta.value()?.parse()?;
            let pattern_str = value.value();
            let message = format!("Invalid format for {}", field_name_str);

            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::pattern(#ref_expr, #pattern_str, #message) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("custom") {
            let value: syn::LitStr = meta.value()?.parse()?;
            let func_name = value.value();
            let func_path: syn::Path = syn::parse_str(&func_name)?;

            validations.push(quote! {
                if let Err(e) = #func_path(#ref_expr) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        Err(meta.error(format!("unknown validator: {}", quote!(#path))))
    })?;

    Ok(ParsedValidateAttr {
        validations,
        skip,
        required,
    })
}
