use std::collections::HashMap;

use chrono::Utc;
use serde_json::json;

use crate::{
    build_decision_index, decision_fingerprint, effective_decision_for_fingerprint,
    evaluate_decision_reuse, merge_decision_indexes, parse_decision_index,
    parse_decision_journal_ndjson, provenance_chain_for_fingerprint, record_fingerprint,
    validate_decision_journal_record, validate_unresolved_decision_record,
    DecisionFingerprintInput, DecisionInvalidationReason, DecisionJournalRecord,
    DecisionReusePolicy, RecordFingerprintInput, UnresolvedDecisionRecord,
};

fn sample_decision_record() -> DecisionJournalRecord {
    DecisionJournalRecord {
        decision_id: underlay_core::Uuid::new_v7(),
        fingerprint: "sha256:1111111111111111111111111111111111111111111111111111111111111111"
            .to_string(),
        decision_type: "migration_record_resolution".to_string(),
        outcome: json!({ "resolved_to": "account_id:1" }),
        confidence: Some(0.98),
        resolver_version: "1.2.0".to_string(),
        prompt_version: "1.0.0".to_string(),
        target_schema_version: "schema_v1".to_string(),
        created_at: Utc::now(),
        provenance: "ai".to_string(),
    }
}

#[test]
fn decision_fingerprint_is_deterministic_for_same_semantics() {
    let left = DecisionFingerprintInput {
        canonical_decision_input: json!({ "b": 2, "a": 1 }),
        decision_type: "map_user".to_string(),
        resolver_version: "1.0.0".to_string(),
        prompt_version: "1.0.0".to_string(),
        target_schema_version: "schema_v1".to_string(),
    };
    let right = DecisionFingerprintInput {
        canonical_decision_input: json!({ "a": 1, "b": 2 }),
        decision_type: "map_user".to_string(),
        resolver_version: "1.0.0".to_string(),
        prompt_version: "1.0.0".to_string(),
        target_schema_version: "schema_v1".to_string(),
    };

    let lhs = decision_fingerprint(&left).expect("left fingerprint");
    let rhs = decision_fingerprint(&right).expect("right fingerprint");
    assert_eq!(lhs, rhs);
}

#[test]
fn record_fingerprint_changes_when_semantic_dependency_changes() {
    let mut deps = HashMap::new();
    deps.insert("plan".to_string(), "starter".to_string());

    let base = RecordFingerprintInput {
        canonical_transform_input: json!({ "legacy_id": 42 }),
        source_identity: "legacy.users:42".to_string(),
        semantic_dependencies: deps.clone(),
    };
    let base_hash = record_fingerprint(&base).expect("base fingerprint");

    deps.insert("plan".to_string(), "pro".to_string());
    let changed = RecordFingerprintInput {
        canonical_transform_input: json!({ "legacy_id": 42 }),
        source_identity: "legacy.users:42".to_string(),
        semantic_dependencies: deps,
    };
    let changed_hash = record_fingerprint(&changed).expect("changed fingerprint");

    assert_ne!(base_hash, changed_hash);
}

#[test]
fn parse_and_validate_decision_journal_ndjson() {
    let record = sample_decision_record();
    let line = serde_json::to_string(&record).expect("serialize");
    let parsed = parse_decision_journal_ndjson(&(line + "\n")).expect("parse ndjson");
    assert_eq!(parsed.len(), 1);
    validate_decision_journal_record(&parsed[0]).expect("journal record should be valid");
}

#[test]
fn build_and_merge_decision_indexes_prefers_newer_entry() {
    let mut record_old = sample_decision_record();
    record_old.created_at = Utc::now() - chrono::Duration::seconds(30);
    let mut record_new = record_old.clone();
    record_new.decision_id = underlay_core::Uuid::new_v7();
    record_new.created_at = Utc::now();

    let index_old = build_decision_index(
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        &[record_old],
    )
    .expect("old index");

    let index_new = build_decision_index(
        "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        &[record_new],
    )
    .expect("new index");

    let merged = merge_decision_indexes(&[index_old.clone(), index_new.clone()]).expect("merge");
    assert_eq!(merged.entries.len(), 1);
    assert_eq!(merged.bundle_digest, index_new.bundle_digest);
    assert_eq!(
        merged
            .entries
            .get("sha256:1111111111111111111111111111111111111111111111111111111111111111")
            .expect("entry")
            .bundle_digest,
        index_new.bundle_digest
    );

    let json = serde_json::to_string(&merged).expect("serialize merged index");
    let parsed = parse_decision_index(&json).expect("parse index");
    assert_eq!(parsed.entries.len(), 1);
}

#[test]
fn evaluate_reuse_strict_requires_exact_versions() {
    let mut record = sample_decision_record();
    let input = DecisionFingerprintInput {
        canonical_decision_input: json!({ "a": 1 }),
        decision_type: record.decision_type.clone(),
        resolver_version: "1.3.0".to_string(),
        prompt_version: record.prompt_version.clone(),
        target_schema_version: record.target_schema_version.clone(),
    };

    let evaluation = evaluate_decision_reuse(
        &record,
        &record.fingerprint,
        &input,
        DecisionReusePolicy::Strict,
        false,
    );
    assert!(!evaluation.reusable);
    assert_eq!(
        evaluation.reason,
        Some(DecisionInvalidationReason::ResolverVersionMismatch)
    );

    record.prompt_version = "1.1.0".to_string();
    let input_exact = DecisionFingerprintInput {
        canonical_decision_input: json!({ "a": 1 }),
        decision_type: record.decision_type.clone(),
        resolver_version: record.resolver_version.clone(),
        prompt_version: "1.2.0".to_string(),
        target_schema_version: record.target_schema_version.clone(),
    };
    let eval_prompt = evaluate_decision_reuse(
        &record,
        &record.fingerprint,
        &input_exact,
        DecisionReusePolicy::Strict,
        false,
    );
    assert!(!eval_prompt.reusable);
    assert_eq!(
        eval_prompt.reason,
        Some(DecisionInvalidationReason::PromptVersionMismatch)
    );
}

#[test]
fn evaluate_reuse_compatible_allows_same_major_versions_and_blocks_plugin_invalidation() {
    let record = sample_decision_record();
    let input = DecisionFingerprintInput {
        canonical_decision_input: json!({ "a": 1 }),
        decision_type: record.decision_type.clone(),
        resolver_version: "1.9.0".to_string(),
        prompt_version: "1.4.2".to_string(),
        target_schema_version: record.target_schema_version.clone(),
    };

    let compatible = evaluate_decision_reuse(
        &record,
        &record.fingerprint,
        &input,
        DecisionReusePolicy::Compatible,
        false,
    );
    assert!(compatible.reusable);

    let invalidated = evaluate_decision_reuse(
        &record,
        &record.fingerprint,
        &input,
        DecisionReusePolicy::Compatible,
        true,
    );
    assert!(!invalidated.reusable);
    assert_eq!(
        invalidated.reason,
        Some(DecisionInvalidationReason::PluginDependencyChanged)
    );
}

#[test]
fn effective_decision_prefers_human_override() {
    let mut ai = sample_decision_record();
    ai.provenance = "ai".to_string();
    ai.outcome = json!({"winner":"ai"});
    ai.created_at = Utc::now();

    let mut human = ai.clone();
    human.decision_id = underlay_core::Uuid::new_v7();
    human.provenance = "human".to_string();
    human.outcome = json!({"winner":"human"});
    human.created_at = Utc::now() - chrono::Duration::seconds(10);

    let records = vec![ai, human];
    let fingerprint = sample_decision_record().fingerprint;
    let selected = effective_decision_for_fingerprint(&records, &fingerprint).expect("selected");
    assert_eq!(selected.provenance, "human");
    assert_eq!(selected.outcome, json!({"winner":"human"}));
}

#[test]
fn provenance_chain_sorted_oldest_first() {
    let mut older = sample_decision_record();
    older.created_at = Utc::now() - chrono::Duration::seconds(30);
    let mut newer = older.clone();
    newer.decision_id = underlay_core::Uuid::new_v7();
    newer.created_at = Utc::now();

    let chain =
        provenance_chain_for_fingerprint(&[newer.clone(), older.clone()], &older.fingerprint);
    assert_eq!(chain.len(), 2);
    assert!(chain[0].created_at <= chain[1].created_at);
}

#[test]
fn unresolved_record_validation_rejects_out_of_range_threshold() {
    let record = UnresolvedDecisionRecord {
        unresolved_id: underlay_core::Uuid::new_v7(),
        run_id: underlay_core::Uuid::new_v7(),
        fingerprint: "sha256:1111111111111111111111111111111111111111111111111111111111111111"
            .to_string(),
        decision_type: "migration_record_resolution".to_string(),
        provenance: "ai".to_string(),
        confidence: Some(0.5),
        threshold: 1.2,
        reason: "low_confidence_ai".to_string(),
        canonical_decision_input: json!({"id":1}),
        created_at: Utc::now(),
    };

    let err = validate_unresolved_decision_record(&record).expect_err("invalid threshold");
    assert!(err.to_string().contains("threshold out of range"));
}
