use underlay_media::{MediaError, MediaResult};

/// Configuration for the PostgreSQL media repository.
#[derive(Clone, Debug)]
pub struct PostgresMediaConfig {
    schema: underlay_db::SqlIdentifier,
    media_table: underlay_db::QualifiedTableName,
    versions_table: underlay_db::QualifiedTableName,
    renditions_table: underlay_db::QualifiedTableName,
    usages_table: underlay_db::QualifiedTableName,
}

impl Default for PostgresMediaConfig {
    fn default() -> Self {
        Self::try_with_schema("media").expect("default media schema should be valid")
    }
}

impl PostgresMediaConfig {
    /// Create a new config with a validated schema name.
    pub fn try_with_schema(schema: impl AsRef<str>) -> MediaResult<Self> {
        let schema = parse_identifier("media schema", schema)?;
        build_config(
            schema,
            "media",
            "media_versions",
            "media_renditions",
            "media_usages",
        )
    }

    /// Set validated table names for the media repository.
    pub fn try_with_tables(
        self,
        media_table: impl AsRef<str>,
        versions_table: impl AsRef<str>,
        renditions_table: impl AsRef<str>,
        usages_table: impl AsRef<str>,
    ) -> MediaResult<Self> {
        build_config(
            self.schema,
            media_table,
            versions_table,
            renditions_table,
            usages_table,
        )
    }

    /// Database schema identifier.
    pub fn schema(&self) -> &underlay_db::SqlIdentifier {
        &self.schema
    }

    /// Fully qualified media table identifier.
    pub fn media_table(&self) -> &underlay_db::QualifiedTableName {
        &self.media_table
    }

    /// Fully qualified media versions table identifier.
    pub fn versions_table(&self) -> &underlay_db::QualifiedTableName {
        &self.versions_table
    }

    /// Fully qualified media renditions table identifier.
    pub fn renditions_table(&self) -> &underlay_db::QualifiedTableName {
        &self.renditions_table
    }

    /// Fully qualified media usages table identifier.
    pub fn usages_table(&self) -> &underlay_db::QualifiedTableName {
        &self.usages_table
    }

    /// Get the fully qualified table name for media.
    pub(super) fn media_fqn(&self) -> MediaResult<String> {
        Ok(self.media_table.quoted())
    }

    /// Get the fully qualified table name for versions.
    pub(super) fn versions_fqn(&self) -> MediaResult<String> {
        Ok(self.versions_table.quoted())
    }

    /// Get the fully qualified table name for renditions.
    pub(super) fn renditions_fqn(&self) -> MediaResult<String> {
        Ok(self.renditions_table.quoted())
    }

    /// Get the fully qualified table name for usages.
    pub(super) fn usages_fqn(&self) -> MediaResult<String> {
        Ok(self.usages_table.quoted())
    }
}

fn build_config(
    schema: underlay_db::SqlIdentifier,
    media_table: impl AsRef<str>,
    versions_table: impl AsRef<str>,
    renditions_table: impl AsRef<str>,
    usages_table: impl AsRef<str>,
) -> MediaResult<PostgresMediaConfig> {
    let media_table = parse_table_name(&schema, "media", media_table)?;
    let versions_table = parse_table_name(&schema, "media versions", versions_table)?;
    let renditions_table = parse_table_name(&schema, "media renditions", renditions_table)?;
    let usages_table = parse_table_name(&schema, "media usages", usages_table)?;

    Ok(PostgresMediaConfig {
        schema,
        media_table,
        versions_table,
        renditions_table,
        usages_table,
    })
}

fn parse_identifier(
    label: &str,
    value: impl AsRef<str>,
) -> MediaResult<underlay_db::SqlIdentifier> {
    underlay_db::SqlIdentifier::parse(value.as_ref())
        .map_err(|err| MediaError::validation(format!("invalid {label} name: {err}")))
}

fn parse_table_name(
    schema: &underlay_db::SqlIdentifier,
    label: &str,
    value: impl AsRef<str>,
) -> MediaResult<underlay_db::QualifiedTableName> {
    let table = parse_identifier(label, value)?;
    underlay_db::QualifiedTableName::from_schema_table(schema.as_str(), table.as_str())
        .map_err(|err| MediaError::validation(format!("invalid {label} table name: {err}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_with_schema_accepts_valid_schema() {
        let config = PostgresMediaConfig::try_with_schema("content").unwrap();
        assert_eq!(config.schema().as_str(), "content");
        assert_eq!(config.media_fqn().unwrap(), "\"content\".\"media\"");
    }

    #[test]
    fn try_with_schema_rejects_invalid_schema() {
        assert!(PostgresMediaConfig::try_with_schema("content.media").is_err());
    }

    #[test]
    fn try_with_tables_accepts_valid_table_names() {
        let config = PostgresMediaConfig::default()
            .try_with_tables("asset", "asset_version", "asset_rendition", "asset_usage")
            .unwrap();

        assert_eq!(config.media_fqn().unwrap(), "\"media\".\"asset\"");
        assert_eq!(
            config.versions_fqn().unwrap(),
            "\"media\".\"asset_version\""
        );
        assert_eq!(
            config.renditions_fqn().unwrap(),
            "\"media\".\"asset_rendition\""
        );
        assert_eq!(config.usages_fqn().unwrap(), "\"media\".\"asset_usage\"");
    }

    #[test]
    fn try_with_tables_rejects_invalid_table_names() {
        assert!(PostgresMediaConfig::default()
            .try_with_tables("asset", "asset-version", "asset_rendition", "asset_usage")
            .is_err());
    }
}
