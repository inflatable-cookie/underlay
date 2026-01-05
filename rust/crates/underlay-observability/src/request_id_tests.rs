#[cfg(test)]
mod tests {
    use crate::request_id::RequestId;

    #[test]
    fn request_id_round_trips_as_header_value() {
        let id = RequestId::new();
        let header = id.to_header_value();
        let parsed = RequestId::from_header_value(&header).expect("should parse");
        assert_eq!(id, parsed);
    }
}
