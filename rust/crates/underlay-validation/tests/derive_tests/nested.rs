use underlay_validation::Validate;

#[derive(Validate)]
struct Address {
    #[validate(required)]
    city: String,
}

#[derive(Validate)]
struct NestedTest {
    #[validate(email)]
    email: String,

    #[validate(nested)]
    address: Address,
}

#[test]
fn test_nested_valid() {
    let req = NestedTest {
        email: "test@example.com".to_string(),
        address: Address {
            city: "London".to_string(),
        },
    };

    assert!(req.validate().is_ok());
}

#[test]
fn test_nested_invalid() {
    let req = NestedTest {
        email: "test@example.com".to_string(),
        address: Address {
            city: "".to_string(),
        },
    };

    let err = req.validate().unwrap_err();
    assert!(err.has_field("address.city"));
}
