use axum::extract::State;
use serde_json::Value;
use underlay_core::SingleResponse;
use underlay_http::{ok, ApiError, ApiResult};
use underlay_media::{
    MediaUsageProvenanceKind, MediaUsageRole, NightfireFieldNameMatcher,
    NightfireMediaUsageExtractor,
};
use underlay_nightfire::{ensure_block_ids, NightfireValue};
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct UpdateLessonBodyRequest {
    body_blocks: NightfireValue,
}

#[derive(serde::Serialize)]
struct LessonDto {
    id: Uuid,
    body_blocks: Value,
}

async fn update_lesson_body(
    State(state): State<AppState>,
    lesson_id: Uuid,
    mut input: UpdateLessonBodyRequest,
) -> ApiResult<SingleResponse<LessonDto>> {
    // 1. Normalize the persisted Nightfire payload before saving it.
    ensure_block_ids(&mut input.body_blocks);

    let body_blocks_json = serde_json::to_value(&input.body_blocks)
        .map_err(|err| ApiError::internal("content.serialize_failed", "Failed to encode body blocks").with_cause(err))?;

    // 2. Persist the exact Nightfire JSON your media extractor will inspect.
    sqlx::query!(
        r#"
        UPDATE lessons
        SET body_blocks = $2::jsonb
        WHERE id = $1
        "#,
        lesson_id,
        body_blocks_json
    )
    .execute(&state.pool)
    .await
    .map_err(|err| ApiError::internal("db.query_failed", "Failed to update lesson").with_cause(err.to_string()))?;

    // 3. Reconcile media_usage from the same persisted Nightfire value.
    let matcher = NightfireFieldNameMatcher::with_common_media_fields()
        .with_field("coverAssetId", MediaUsageRole::Primary);

    let extractor = NightfireMediaUsageExtractor::new(
        "lesson",
        Some(lesson_id),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        matcher,
    );

    extractor
        .extract_and_sync(&state.media_repo, &input.body_blocks)
        .await
        .map_err(|err| ApiError::internal("media.sync_failed", "Failed to sync media usage").with_cause(err.to_string()))?;

    Ok(ok(LessonDto {
        id: lesson_id,
        body_blocks: serde_json::to_value(&input.body_blocks)
            .map_err(|err| ApiError::internal("content.serialize_failed", "Failed to encode body blocks").with_cause(err))?,
    }))
}
