### Fixed

- A failed `Verify` stage no longer persists a completed resume checkpoint, so a resumed migration run re-runs verification instead of skipping past it with the failed snapshot.
- `#[validate(required)]` on `Option<T>` fields compiles for any payload type and enforces presence. Blank-string rejection still applies to `Option<String>` and `String`.
- `validators::email` rejects malformed domain labels such as `user@..com`, `user@-example.com`, and `user@example-.com`.
- `ValidationError::add_field` keeps the first non-empty error code, so an empty code no longer blocks a later one.
- `validators::pattern` returns the structured code `pattern.invalid_regex` for an invalid regex pattern.
- Pool-timeout, pool-closed, and I/O database errors return generic client messages without operational hints.
- Query column names are validated before SQL interpolation. Mapped columns that are not a plain identifier or one `alias.column` pair are treated as unmapped.
