use underlay_validation::Validate;

#[derive(Validate)]
struct CollectionTest {
    #[validate(not_empty)]
    items: Vec<String>,

    #[validate(collection_length(min = 2, max = 5))]
    tags: Vec<String>,
}

#[test]
fn test_collection_validators_valid() {
    let req = CollectionTest {
        items: vec!["a".to_string()],
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    };

    assert!(req.validate().is_ok());
}

#[test]
fn test_collection_validators_invalid() {
    let req = CollectionTest {
        items: vec![],
        tags: vec!["tag1".to_string()],
    };

    let err = req.validate().unwrap_err();
    assert!(err.has_field("items"));
    assert!(err.has_field("tags"));
}
