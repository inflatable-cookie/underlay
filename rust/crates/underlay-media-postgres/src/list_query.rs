use underlay_media::{ListMediaParams, MediaResult};

use super::PostgresMediaConfig;

pub(super) fn build_list_media_query(
    config: &PostgresMediaConfig,
    params: &ListMediaParams,
) -> MediaResult<String> {
    let mut conditions = Vec::new();

    if !params.include_deleted {
        conditions.push("m.deleted_at IS NULL".to_string());
    }

    let kind_bind_index = 1;
    if params.kind.is_some() {
        conditions.push(format!("m.kind = ${kind_bind_index}"));
    }

    let visibility_bind_index = kind_bind_index + usize::from(params.kind.is_some());
    if params.visibility.is_some() {
        conditions.push(format!("m.visibility = ${visibility_bind_index}"));
    }

    let search_bind_index = visibility_bind_index + usize::from(params.visibility.is_some());
    if params.search.is_some() {
        conditions.push(format!(
            "(m.title ILIKE ${search_bind_index} OR m.original_filename ILIKE ${search_bind_index})"
        ));
    }

    if params.unused_only {
        conditions.push(format!(
            "NOT EXISTS (SELECT 1 FROM {} u WHERE u.media_id = m.id)",
            config.usages_fqn()?
        ));
    }

    let where_clause = if conditions.is_empty() {
        "1=1".to_string()
    } else {
        conditions.join(" AND ")
    };

    let limit_clause = params
        .limit
        .map(|limit| format!("LIMIT {limit}"))
        .unwrap_or_default();

    Ok(format!(
        r#"
            SELECT m.id, m.kind, m.visibility, m.title, m.original_filename,
                   m.current_version_id, m.created_at, m.updated_at, m.deleted_at,
                   v.byte_size, v.mime_type,
                   r.object_key as thumbnail_object_key
            FROM {} m
            LEFT JOIN {} v ON v.id = m.current_version_id
            LEFT JOIN {} r ON r.media_version_id = v.id AND r.kind = 'thumbnail'
            WHERE {}
            ORDER BY m.updated_at DESC, m.id DESC
            {}
            "#,
        config.media_fqn()?,
        config.versions_fqn()?,
        config.renditions_fqn()?,
        where_clause,
        limit_clause
    ))
}

#[cfg(test)]
mod tests {
    use underlay_media::{ListMediaParams, MediaKind, MediaVisibility};

    use super::*;

    #[test]
    fn builds_default_list_query() {
        let query =
            build_list_media_query(&PostgresMediaConfig::default(), &ListMediaParams::default())
                .unwrap();

        assert!(query.contains(r#"FROM "media"."media" m"#));
        assert!(query.contains("m.deleted_at IS NULL"));
        assert!(!query.contains("LIMIT"));
    }

    #[test]
    fn builds_filter_bind_indices_in_repository_bind_order() {
        let params = ListMediaParams {
            kind: Some(MediaKind::Image),
            visibility: Some(MediaVisibility::Restricted),
            search: Some("hero".to_string()),
            unused_only: true,
            limit: Some(20),
            ..ListMediaParams::default()
        };

        let query = build_list_media_query(&PostgresMediaConfig::default(), &params).unwrap();

        assert!(query.contains("m.kind = $1"));
        assert!(query.contains("m.visibility = $2"));
        assert!(query.contains("(m.title ILIKE $3 OR m.original_filename ILIKE $3)"));
        assert!(query.contains(r#"FROM "media"."media_usages" u"#));
        assert!(query.contains("LIMIT 20"));
    }

    #[test]
    fn rejects_invalid_configured_table_names() {
        let config = PostgresMediaConfig {
            media_table: "media;DROP".to_string(),
            ..PostgresMediaConfig::default()
        };

        assert!(build_list_media_query(&config, &ListMediaParams::default()).is_err());
    }
}
