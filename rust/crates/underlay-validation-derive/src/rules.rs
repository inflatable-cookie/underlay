use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, Ident, Result as SynResult};

pub(crate) fn parse_validate_attr(
    attr: &Attribute,
    field_name: &Ident,
    field_name_str: &str,
) -> SynResult<Vec<TokenStream>> {
    let mut validations = Vec::new();

    attr.parse_nested_meta(|meta| {
        let path = &meta.path;

        // Simple validators (no arguments)
        if path.is_ident("skip") {
            return Ok(());
        }

        if path.is_ident("email") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::email(&self.#field_name) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("url") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::url(&self.#field_name) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("uuid") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::uuid(&self.#field_name) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("required") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::required(&self.#field_name) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("alphanumeric") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::alphanumeric(&self.#field_name) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("username") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::username(&self.#field_name) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("slug") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::slug(&self.#field_name) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("positive") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::positive(self.#field_name) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("non_negative") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::non_negative(self.#field_name) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("not_empty") {
            validations.push(quote! {
                if let Err(e) = ::underlay_validation::validators::not_empty(&self.#field_name) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("nested") {
            validations.push(quote! {
                if let Err(nested_errors) = ::underlay_validation::Validate::validate(&self.#field_name) {
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
                if let Err(e) = ::underlay_validation::validators::length(&self.#field_name, #min_expr, #max_expr) {
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
                if let Err(e) = ::underlay_validation::validators::range(self.#field_name, #min_expr, #max_expr) {
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
                if let Err(e) = ::underlay_validation::validators::collection_length(&self.#field_name, #min_expr, #max_expr) {
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
                if let Err(e) = ::underlay_validation::validators::pattern(&self.#field_name, #pattern_str, #message) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        if path.is_ident("custom") {
            let value: syn::LitStr = meta.value()?.parse()?;
            let func_name = value.value();
            let func_ident: Ident = syn::parse_str(&func_name)?;

            validations.push(quote! {
                if let Err(e) = #func_ident(&self.#field_name) {
                    errors.add_field(#field_name_str, e);
                }
            });
            return Ok(());
        }

        Err(meta.error(format!("unknown validator: {}", quote!(#path))))
    })?;

    Ok(validations)
}
