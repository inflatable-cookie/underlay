    use super::impl_validate;
    use syn::parse_quote;

    fn normalize(tokens: proc_macro2::TokenStream) -> String {
        tokens
            .to_string()
            .split_whitespace()
            .collect::<String>()
            .to_lowercase()
    }

    #[test]
    fn derive_rejects_enums() {
        let input: syn::DeriveInput = parse_quote! {
            enum Example {
                A
            }
        };

        let err = impl_validate(&input).expect_err("enums should be rejected");
        assert!(err.to_string().contains("structs, not enums"));
    }

    #[test]
    fn derive_rejects_tuple_structs() {
        let input: syn::DeriveInput = parse_quote! {
            struct Example(String);
        };

        let err = impl_validate(&input).expect_err("tuple structs should be rejected");
        assert!(err.to_string().contains("named fields"));
    }

    #[test]
    fn derive_generates_email_and_length_validations() {
        let input: syn::DeriveInput = parse_quote! {
            struct CreateUser {
                #[validate(email, length(min = 3, max = 200))]
                email: String,
            }
        };

        let tokens = impl_validate(&input).expect("derive generation should succeed");
        let rendered = normalize(tokens);

        assert!(rendered.contains("impl::underlay_validation::validateforcreateuser"));
        assert!(rendered.contains("validators::email(&self.email)"));
        assert!(rendered.contains("validators::length(&self.email,some(3),some(200))"));
    }

    #[test]
    fn derive_generates_nested_and_custom_validations() {
        let input: syn::DeriveInput = parse_quote! {
            struct Wrapper {
                #[validate(nested)]
                nested: Inner,
                #[validate(custom = "check_name")]
                name: String,
            }
        };

        let tokens = impl_validate(&input).expect("derive generation should succeed");
        let rendered = normalize(tokens);

        assert!(rendered.contains("validate::validate(&self.nested)"));
        assert!(rendered.contains("check_name(&self.name)"));
        assert!(rendered.contains("errors.merge_nested(\"nested\",nested_errors)"));
    }

    #[test]
    fn derive_rejects_unknown_validator() {
        let input: syn::DeriveInput = parse_quote! {
            struct Example {
                #[validate(nope)]
                value: String,
            }
        };

        let err = impl_validate(&input).expect_err("unknown validator should fail");
        assert!(err.to_string().contains("unknown validator"));
    }