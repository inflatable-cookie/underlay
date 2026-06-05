use underlay_media::{MediaError, MediaResult};

/// Configuration for the PostgreSQL media repository.
#[derive(Clone, Debug)]
pub struct PostgresMediaConfig {
    /// Database schema name.
    pub schema: String,
    /// Table name for media items.
    pub media_table: String,
    /// Table name for media versions.
    pub versions_table: String,
    /// Table name for media renditions.
    pub renditions_table: String,
    /// Table name for media usages.
    pub usages_table: String,
}

impl Default for PostgresMediaConfig {
    fn default() -> Self {
        Self {
            schema: "media".to_string(),
            media_table: "media".to_string(),
            versions_table: "media_versions".to_string(),
            renditions_table: "media_renditions".to_string(),
            usages_table: "media_usages".to_string(),
        }
    }
}

impl PostgresMediaConfig {
    /// Create a new config with the given schema name.
    pub fn with_schema(schema: impl Into<String>) -> Self {
        Self {
            schema: schema.into(),
            ..Default::default()
        }
    }

    /// Create a new config with a validated schema name.
    pub fn try_with_schema(schema: impl AsRef<str>) -> MediaResult<Self> {
        let schema = underlay_db::SqlIdentifier::parse(schema.as_ref())
            .map_err(|err| MediaError::validation(format!("invalid media schema name: {err}")))?;
        Ok(Self::with_schema(schema.as_str()))
    }

    /// Set validated table names for the media repository.
    pub fn try_with_tables(
        mut self,
        media_table: impl AsRef<str>,
        versions_table: impl AsRef<str>,
        renditions_table: impl AsRef<str>,
        usages_table: impl AsRef<str>,
    ) -> MediaResult<Self> {
        self.media_table = parse_table_name("media", media_table)?;
        self.versions_table = parse_table_name("media versions", versions_table)?;
        self.renditions_table = parse_table_name("media renditions", renditions_table)?;
        self.usages_table = parse_table_name("media usages", usages_table)?;
        Ok(self)
    }

    /// Get the fully qualified table name for media.
    pub(super) fn media_fqn(&self) -> MediaResult<String> {
        underlay_db::format_schema_table(&self.schema, &self.media_table)
            .map_err(|err| MediaError::validation(format!("invalid media table name: {err}")))
    }

    /// Get the fully qualified table name for versions.
    pub(super) fn versions_fqn(&self) -> MediaResult<String> {
        underlay_db::format_schema_table(&self.schema, &self.versions_table).map_err(|err| {
            MediaError::validation(format!("invalid media versions table name: {err}"))
        })
    }

    /// Get the fully qualified table name for renditions.
    pub(super) fn renditions_fqn(&self) -> MediaResult<String> {
        underlay_db::format_schema_table(&self.schema, &self.renditions_table).map_err(|err| {
            MediaError::validation(format!("invalid media renditions table name: {err}"))
        })
    }

    /// Get the fully qualified table name for usages.
    pub(super) fn usages_fqn(&self) -> MediaResult<String> {
        underlay_db::format_schema_table(&self.schema, &self.usages_table).map_err(|err| {
            MediaError::validation(format!("invalid media usages table name: {err}"))
        })
    }
}

fn parse_table_name(label: &str, value: impl AsRef<str>) -> MediaResult<String> {
    let table = underlay_db::SqlIdentifier::parse(value.as_ref())
        .map_err(|err| MediaError::validation(format!("invalid {label} table name: {err}")))?;
    Ok(table.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_with_schema_accepts_valid_schema() {
        let config = PostgresMediaConfig::try_with_schema("content").unwrap();
        assert_eq!(config.schema, "content");
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
