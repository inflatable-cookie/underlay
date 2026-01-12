//! Derive macro for the `Validate` trait.
//!
//! This crate provides a `#[derive(Validate)]` macro that automatically generates
//! validation code based on field attributes.
//!
//! # Example
//!
//! ```rust,ignore
//! use underlay_validation::Validate;
//!
//! #[derive(Validate)]
//! struct CreateUserRequest {
//!     #[validate(email)]
//!     email: String,
//!
//!     #[validate(length(min = 8, max = 100))]
//!     password: String,
//!
//!     #[validate(range(min = 18, max = 120))]
//!     age: i32,
//!
//!     #[validate(required)]
//!     name: String,
//! }
//! ```

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Expr, Field, Fields, Ident,
    Result as SynResult,
};

/// Derive the `Validate` trait for a struct.
///
/// # Supported Attributes
///
/// - `#[validate(email)]` - Validate as email
/// - `#[validate(url)]` - Validate as URL
/// - `#[validate(uuid)]` - Validate as UUID
/// - `#[validate(required)]` - Non-empty string
/// - `#[validate(length(min = N, max = M))]` - String length bounds
/// - `#[validate(range(min = N, max = M))]` - Numeric range bounds
/// - `#[validate(pattern = "regex")]` - Custom regex
/// - `#[validate(custom = "function_name")]` - Custom validator function
/// - `#[validate(nested)]` - Validate nested struct
/// - `#[validate(skip)]` - Skip validation for this field
#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match impl_validate(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn impl_validate(input: &DeriveInput) -> SynResult<TokenStream2> {
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

    let field_validations: Vec<TokenStream2> = fields
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

fn generate_field_validation(field: &Field) -> SynResult<Option<TokenStream2>> {
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

fn parse_validate_attr(
    attr: &Attribute,
    field_name: &Ident,
    field_name_str: &str,
) -> SynResult<Vec<TokenStream2>> {
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
            let mut min_val: Option<TokenStream2> = None;
            let mut max_val: Option<TokenStream2> = None;

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
            let mut min_val: Option<TokenStream2> = None;
            let mut max_val: Option<TokenStream2> = None;

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
            let mut min_val: Option<TokenStream2> = None;
            let mut max_val: Option<TokenStream2> = None;

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
