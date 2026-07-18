//! Integration tests for the Postgres media repository against a real Postgres.
//!
//! `#[ignore]`d by default (needs a database). Run with:
//!
//! ```bash
//! UNDERLAY_TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/postgres \
//!   cargo test -p underlay-media-postgres --lib -- --ignored
//! ```
//!
//! The fixture builds the four media tables from the *adapter's* column usage
//! (not a consumer migration — those have drifted), so the tests double as a
//! schema-contract check on the hand-written SQL.

use underlay_media::{
    BlobObjectKey, CreateMediaInput, FinalizeUploadInput, ListMediaParams, MediaKind,
    MediaRepository, MediaUsage, MediaUsageRepository, MediaVersionState, MediaVisibility,
    UpdateMediaInput,
};
use underlay_testing::TestDb;
use uuid::Uuid;

use crate::{PostgresMediaConfig, PostgresMediaRepository};

struct Fixture {
    _db: TestDb,
    repo: PostgresMediaRepository,
}

/// Create the media / versions / renditions / usages tables in this test's
/// isolated schema with exactly the columns the adapter reads and writes.
async fn setup() -> Fixture {
    let db = TestDb::new().await;
    let schema = db.schema_name().to_string();

    let ddl = format!(
        r#"
        CREATE TABLE {schema}.media (
            id uuid PRIMARY KEY,
            kind text NOT NULL,
            visibility text NOT NULL,
            title text NOT NULL,
            original_filename text NULL,
            alt_text text NULL,
            current_version_id uuid NULL,
            created_at timestamptz NOT NULL DEFAULT now(),
            updated_at timestamptz NOT NULL DEFAULT now(),
            deleted_at timestamptz NULL,
            created_by uuid NULL
        );
        CREATE TABLE {schema}.media_versions (
            id uuid PRIMARY KEY,
            media_id uuid NOT NULL REFERENCES {schema}.media(id) ON DELETE CASCADE,
            state text NOT NULL DEFAULT 'uploading',
            byte_size bigint NULL,
            mime_type text NULL,
            sha256 text NULL,
            width int NULL,
            height int NULL,
            storage_provider text NULL,
            bucket text NULL,
            object_key text NULL,
            created_at timestamptz NOT NULL DEFAULT now(),
            created_by uuid NULL
        );
        CREATE TABLE {schema}.media_renditions (
            id uuid PRIMARY KEY,
            media_version_id uuid NOT NULL REFERENCES {schema}.media_versions(id) ON DELETE CASCADE,
            kind text NOT NULL,
            byte_size bigint NOT NULL DEFAULT 0,
            mime_type text NOT NULL DEFAULT '',
            width int NULL,
            height int NULL,
            storage_provider text NOT NULL DEFAULT '',
            bucket text NOT NULL DEFAULT '',
            object_key text NOT NULL,
            created_at timestamptz NOT NULL DEFAULT now()
        );
        CREATE TABLE {schema}.media_usages (
            id uuid PRIMARY KEY,
            media_id uuid NOT NULL REFERENCES {schema}.media(id) ON DELETE CASCADE,
            used_by_type text NOT NULL,
            used_by_id uuid NULL,
            field text NOT NULL,
            created_at timestamptz NOT NULL DEFAULT now(),
            UNIQUE (media_id, used_by_type, used_by_id, field)
        );
        "#
    );

    // Execute statement-by-statement (simple executor runs one at a time).
    for stmt in ddl.split(';') {
        let stmt = stmt.trim();
        if stmt.is_empty() {
            continue;
        }
        sqlx::query(stmt)
            .execute(db.pool())
            .await
            .expect("create media schema");
    }

    let config = PostgresMediaConfig::try_with_schema(&schema).expect("valid schema");
    let repo = PostgresMediaRepository::with_config(db.pool().clone(), config);
    Fixture { _db: db, repo }
}

fn new_media_input(title: &str) -> CreateMediaInput {
    CreateMediaInput {
        kind: MediaKind::Image,
        visibility: MediaVisibility::Restricted,
        title: title.to_string(),
        original_filename: Some(format!("{title}.png")),
        alt_text: Some(format!("{title} alt")),
    }
}

fn default_list_params() -> ListMediaParams {
    ListMediaParams {
        kind: None,
        visibility: None,
        search: None,
        include_deleted: false,
        unused_only: false,
        limit: None,
        cursor: None,
    }
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn media_create_get_update_round_trip() {
    let fx = setup().await;
    let creator = Uuid::now_v7();

    let media = fx
        .repo
        .create_media(new_media_input("Cover"), Some(creator))
        .await
        .expect("create_media");
    assert_eq!(media.title, "Cover");
    assert_eq!(media.kind, MediaKind::Image);
    assert_eq!(media.visibility, MediaVisibility::Restricted);
    assert!(media.current_version_id.is_none());

    let fetched = fx
        .repo
        .get_media(media.id)
        .await
        .expect("get")
        .expect("some");
    assert_eq!(fetched.id, media.id);

    let updated = fx
        .repo
        .update_media(
            media.id,
            UpdateMediaInput {
                title: "Renamed".to_string(),
                original_filename: Some("renamed.png".to_string()),
                visibility: MediaVisibility::Public,
                alt_text: Some("new alt".to_string()),
            },
            None,
        )
        .await
        .expect("update_media");
    assert_eq!(updated.title, "Renamed");
    assert_eq!(updated.visibility, MediaVisibility::Public);

    // Unknown id -> None.
    assert!(fx
        .repo
        .get_media(underlay_media::MediaId(Uuid::now_v7()))
        .await
        .expect("get missing")
        .is_none());
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn version_lifecycle_and_current_version() {
    let fx = setup().await;
    let media = fx
        .repo
        .create_media(new_media_input("Doc"), None)
        .await
        .expect("create_media");

    let version = fx
        .repo
        .create_version(media.id, None)
        .await
        .expect("create_version");
    assert_eq!(version.state, MediaVersionState::Uploading);

    let finalized = fx
        .repo
        .finalize_version(
            version.id,
            FinalizeUploadInput {
                byte_size: 2048,
                mime_type: "image/png".to_string(),
                sha256_hash: "a".repeat(64),
                storage_provider: "s3".to_string(),
                bucket: "media".to_string(),
                object_key: BlobObjectKey::parse("media/doc.png").expect("key"),
                width: Some(640),
                height: Some(480),
            },
        )
        .await
        .expect("finalize_version");
    assert_eq!(finalized.state, MediaVersionState::Ready);

    fx.repo
        .set_current_version(media.id, version.id)
        .await
        .expect("set_current_version");

    let reloaded = fx
        .repo
        .get_media(media.id)
        .await
        .expect("get")
        .expect("some");
    assert_eq!(reloaded.current_version_id, Some(version.id));

    // Dedup lookup by hash finds the media.
    let by_hash = fx
        .repo
        .find_by_hash(&"a".repeat(64))
        .await
        .expect("find_by_hash");
    assert_eq!(by_hash.map(|m| m.id), Some(media.id));

    let versions = fx
        .repo
        .list_versions(media.id)
        .await
        .expect("list_versions");
    assert_eq!(versions.len(), 1);
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn soft_delete_trash_and_restore() {
    let fx = setup().await;
    let media = fx
        .repo
        .create_media(new_media_input("Temp"), None)
        .await
        .expect("create_media");

    assert!(fx
        .repo
        .soft_delete_media(media.id, None)
        .await
        .expect("soft_delete"));

    // Gone from the live list, present in trash.
    let live = fx
        .repo
        .list_media(default_list_params())
        .await
        .expect("list");
    assert!(live.iter().all(|m| m.id != media.id));
    let trash = fx.repo.list_trash().await.expect("list_trash");
    assert!(trash.iter().any(|m| m.id == media.id));

    assert!(fx.repo.restore_media(media.id).await.expect("restore"));
    let live_again = fx
        .repo
        .list_media(default_list_params())
        .await
        .expect("list");
    assert!(live_again.iter().any(|m| m.id == media.id));

    // Hard delete removes it entirely.
    assert!(fx
        .repo
        .hard_delete_media(media.id)
        .await
        .expect("hard_delete"));
    assert!(fx.repo.get_media(media.id).await.expect("get").is_none());
}

#[tokio::test]
#[ignore = "requires a Postgres test database (UNDERLAY_TEST_DATABASE_URL or Docker)"]
async fn usage_tracking_and_counts() {
    let fx = setup().await;
    let media = fx
        .repo
        .create_media(new_media_input("Shared"), None)
        .await
        .expect("create_media");

    assert!(!fx.repo.is_media_used(media.id).await.expect("is_used"));
    assert_eq!(fx.repo.get_usage_count(media.id).await.expect("count"), 0);

    let entity = Uuid::now_v7();
    let usage = MediaUsage {
        id: Uuid::now_v7(),
        media_id: media.id,
        entity_type: "article".to_string(),
        entity_id: entity,
        field_name: "cover_image".to_string(),
        created_at: chrono::Utc::now(),
    };
    fx.repo.track_usage(&usage).await.expect("track_usage");
    // Idempotent (ON CONFLICT DO NOTHING).
    fx.repo
        .track_usage(&usage)
        .await
        .expect("track_usage again");

    assert!(fx.repo.is_media_used(media.id).await.expect("is_used"));
    assert_eq!(fx.repo.get_usage_count(media.id).await.expect("count"), 1);

    let usages = fx.repo.list_usages(media.id).await.expect("list_usages");
    assert_eq!(usages.len(), 1);
    assert_eq!(usages[0].entity_type, "article");

    fx.repo
        .remove_usage(media.id, "article", entity, "cover_image")
        .await
        .expect("remove_usage");
    assert_eq!(fx.repo.get_usage_count(media.id).await.expect("count"), 0);
}
