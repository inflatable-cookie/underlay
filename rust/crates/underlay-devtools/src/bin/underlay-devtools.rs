use std::path::PathBuf;

fn print_usage() {
    eprintln!(
        "underlay-devtools\n\nUSAGE:\n  underlay-devtools sync-migrations --target <DIR> [--dry-run]\n  underlay-devtools migration bundle build --output <FILE> --source-system <NAME> --target-schema-version <VERSION> [--media-dir <DIR>]\n  underlay-devtools migration bundle publish --bundle <FILE> --oci-ref <REF>\n  underlay-devtools migration bundle pull --oci-ref <REF> --output <DIR>\n  underlay-devtools migration run --bundle <REF@DIGEST> --output <DIR>\n  underlay-devtools migration report governance --input <FILE> [--limit <N>]\n  underlay-devtools migration report policy --input <FILE>\n  underlay-devtools migration report drift --input <FILE> [--max-unresolved <N>] [--max-governance <N>] [--max-lineage <N>] [--decision-index <FILE>] [--decision-journal <FILE>] [--expected-bundle-digest <sha256:...>]\n  underlay-devtools migration report recovery --input <FILE>\n  underlay-devtools migration report verify --input <FILE> [--output-dir <DIR>]\n  underlay-devtools migration report integrity --input <FILE>\n  underlay-devtools migration report audit --input <FILE|DIR> [--output-dir <DIR>]\n  underlay-devtools seed bundle build --source <DIR> --output <FILE>\n  underlay-devtools seed bundle publish --bundle <FILE> --oci-ref <REF>\n  underlay-devtools seed bundle pull --oci-ref <REF> --output <DIR>\n\nCOMMANDS:\n  sync-migrations    Copy Underlay-owned SQL migrations into an app's migrations directory\n  migration bundle   Manage migration OCI bundle packages (build/publish/pull)\n  migration run      Prepare and validate a digest-pinned migration replay input\n  migration report   Summarize migration governance, policy, drift, integrity, and recovery outcomes\n  seed bundle        Package seed-data SQL directories as OCI bundles (build/publish/pull)\n\nOPTIONS:\n  --target <DIR>              Target migrations directory (must exist)\n  --dry-run                   Print what would be written, without writing\n  --output <FILE|DIR>         Output file (build) or output directory (pull/run)\n  --source <DIR>              Source seed-bundle directory containing manifest.json + SQL files\n  --source-system <NAME>      Legacy source identifier for bundle metadata\n  --target-schema-version <V> Target schema version for bundle metadata\n  --media-dir <DIR>           Optional directory of media files to embed in media shard payload\n  --bundle <FILE|REF>         Bundle file path (publish) or digest-pinned OCI reference (run)\n  --oci-ref <REF>             OCI reference (tag or digest)\n  --input <FILE>              Decide stage JSON, policy JSON, or full pipeline run report JSON\n  --output-dir <DIR>          Directory for generated verification/audit artifact output\n  --decision-index <FILE>     Decision index JSON for lineage drift checks\n  --decision-journal <FILE>   Decision journal NDJSON for lineage drift checks\n  --expected-bundle-digest <D> Expected bundle digest for index linkage checks\n  --limit <N>                 Maximum governance issue examples to print (default 5)\n  --max-unresolved <N>        Drift threshold for unresolved decisions (default 0)\n  --max-governance <N>        Drift threshold for governance issues (default 0)\n  --max-lineage <N>           Drift threshold for lineage mismatches (default 0)\n"
    );
}

fn main() {
    let mut args = std::env::args().skip(1);

    let Some(cmd) = args.next() else {
        print_usage();
        std::process::exit(2);
    };

    if cmd == "--help" || cmd == "-h" {
        print_usage();
        return;
    }

    match cmd.as_str() {
        "sync-migrations" => run_sync_migrations(args),
        "migration" => run_migration(args),
        "seed" => run_seed(args),
        other => {
            eprintln!("unknown command: {other}");
            print_usage();
            std::process::exit(2);
        }
    }
}

fn run_sync_migrations(mut args: impl Iterator<Item = String>) {
    let mut target: Option<PathBuf> = None;
    let mut dry_run = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--target" => {
                let Some(v) = args.next() else {
                    eprintln!("--target requires a value");
                    std::process::exit(2);
                };
                target = Some(PathBuf::from(v));
            }
            "--dry-run" => {
                dry_run = true;
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(target) = target else {
        eprintln!("missing --target");
        print_usage();
        std::process::exit(2);
    };

    match underlay_devtools::sync_migrations(&target, dry_run) {
        Ok(report) => {
            for path in &report.written {
                if dry_run {
                    println!("would write {}", path.display());
                } else {
                    println!("wrote {}", path.display());
                }
            }
            for path in &report.skipped {
                println!("skipped {}", path.display());
            }
        }
        Err(err) => {
            eprintln!("sync-migrations failed: {err}");
            std::process::exit(1);
        }
    }
}

fn run_migration(mut args: impl Iterator<Item = String>) {
    let Some(subcommand) = args.next() else {
        eprintln!("missing migration subcommand");
        print_usage();
        std::process::exit(2);
    };

    if subcommand == "run" {
        run_migration_run(args);
        return;
    }

    if subcommand == "report" {
        run_migration_report(args);
        return;
    }

    if subcommand != "bundle" {
        eprintln!("unknown migration subcommand: {subcommand}");
        print_usage();
        std::process::exit(2);
    }

    let Some(action) = args.next() else {
        eprintln!("missing migration bundle action");
        print_usage();
        std::process::exit(2);
    };

    match action.as_str() {
        "build" => run_bundle_build(args),
        "publish" => run_bundle_publish(args),
        "pull" => run_bundle_pull(args),
        other => {
            eprintln!("unknown migration bundle action: {other}");
            print_usage();
            std::process::exit(2);
        }
    }
}

fn run_migration_report(mut args: impl Iterator<Item = String>) {
    let Some(report_type) = args.next() else {
        eprintln!("missing migration report type");
        print_usage();
        std::process::exit(2);
    };

    match report_type.as_str() {
        "governance" => run_migration_report_governance(args),
        "policy" => run_migration_report_policy(args),
        "drift" => run_migration_report_drift(args),
        "recovery" => run_migration_report_recovery(args),
        "verify" => run_migration_report_verify(args),
        "integrity" => run_migration_report_integrity(args),
        "audit" => run_migration_report_audit(args),
        _ => {
            eprintln!("unknown migration report type: {report_type}");
            print_usage();
            std::process::exit(2);
        }
    }
}

fn run_migration_report_policy(mut args: impl Iterator<Item = String>) {
    let mut input: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                let Some(v) = args.next() else {
                    eprintln!("--input requires a value");
                    std::process::exit(2);
                };
                input = Some(PathBuf::from(v));
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(input_file) = input else {
        eprintln!("missing --input");
        std::process::exit(2);
    };

    let policy = match underlay_devtools::load_governance_policy(&input_file) {
        Ok(policy) => policy,
        Err(err) => {
            eprintln!("migration report policy failed: {err}");
            std::process::exit(1);
        }
    };
    let report = underlay_devtools::build_policy_report(&policy);
    for line in underlay_devtools::format_policy_summary(&report) {
        println!("policy {line}");
    }
}

fn run_migration_report_integrity(mut args: impl Iterator<Item = String>) {
    let mut input: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                let Some(v) = args.next() else {
                    eprintln!("--input requires a value");
                    std::process::exit(2);
                };
                input = Some(PathBuf::from(v));
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(input_file) = input else {
        eprintln!("missing --input");
        std::process::exit(2);
    };

    let report = match underlay_devtools::load_pipeline_run_report(&input_file) {
        Ok(report) => report,
        Err(err) => {
            eprintln!("migration report integrity failed: {err}");
            std::process::exit(1);
        }
    };
    let artifact = underlay_devtools::build_integrity_report(&report);
    for line in underlay_devtools::format_integrity_summary(&artifact) {
        println!("integrity {line}");
    }
}

fn run_migration_report_audit(mut args: impl Iterator<Item = String>) {
    let mut input: Option<PathBuf> = None;
    let mut output_dir: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                let Some(v) = args.next() else {
                    eprintln!("--input requires a value");
                    std::process::exit(2);
                };
                input = Some(PathBuf::from(v));
            }
            "--output-dir" => {
                let Some(v) = args.next() else {
                    eprintln!("--output-dir requires a value");
                    std::process::exit(2);
                };
                output_dir = Some(PathBuf::from(v));
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(input_path) = input else {
        eprintln!("missing --input");
        std::process::exit(2);
    };
    let output_dir = output_dir.unwrap_or_else(|| PathBuf::from(".underlay-migration"));

    let report = match underlay_devtools::load_pipeline_run_report_from_path(&input_path) {
        Ok(report) => report,
        Err(err) => {
            eprintln!("migration report audit failed: {err}");
            std::process::exit(1);
        }
    };
    let artifact = underlay_devtools::build_audit_report(&report);
    let written = match underlay_devtools::write_audit_artifact(&output_dir, &artifact) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("migration report audit failed: {err}");
            std::process::exit(1);
        }
    };

    println!("audit artifact {}", written.display());
    for line in underlay_devtools::format_audit_summary(&artifact) {
        println!("audit {line}");
    }
}

fn run_migration_report_governance(mut args: impl Iterator<Item = String>) {
    let mut input: Option<PathBuf> = None;
    let mut limit: usize = 5;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                let Some(v) = args.next() else {
                    eprintln!("--input requires a value");
                    std::process::exit(2);
                };
                input = Some(PathBuf::from(v));
            }
            "--limit" => {
                let Some(v) = args.next() else {
                    eprintln!("--limit requires a value");
                    std::process::exit(2);
                };
                limit = parse_usize_arg("--limit", &v);
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(input_file) = input else {
        eprintln!("missing --input");
        std::process::exit(2);
    };

    let decide = match underlay_devtools::load_decide_stage_output(&input_file) {
        Ok(decide) => decide,
        Err(err) => {
            eprintln!("migration report governance failed: {err}");
            std::process::exit(1);
        }
    };

    for line in underlay_devtools::format_decision_invalidation_report(&decide) {
        println!("invalidations {line}");
    }
    for line in underlay_devtools::format_decision_governance_report(&decide) {
        println!("governance {line}");
    }
    for issue in underlay_devtools::top_governance_issues(&decide, limit) {
        println!(
            "issue artifact={} code={} fingerprint={} message={}",
            issue.artifact, issue.code, issue.fingerprint, issue.message
        );
    }
}

fn run_migration_report_drift(mut args: impl Iterator<Item = String>) {
    let mut input: Option<PathBuf> = None;
    let mut max_unresolved: usize = 0;
    let mut max_governance: usize = 0;
    let mut max_lineage: usize = 0;
    let mut decision_index: Option<PathBuf> = None;
    let mut decision_journal: Option<PathBuf> = None;
    let mut expected_bundle_digest: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                let Some(v) = args.next() else {
                    eprintln!("--input requires a value");
                    std::process::exit(2);
                };
                input = Some(PathBuf::from(v));
            }
            "--max-unresolved" => {
                let Some(v) = args.next() else {
                    eprintln!("--max-unresolved requires a value");
                    std::process::exit(2);
                };
                max_unresolved = parse_usize_arg("--max-unresolved", &v);
            }
            "--max-governance" => {
                let Some(v) = args.next() else {
                    eprintln!("--max-governance requires a value");
                    std::process::exit(2);
                };
                max_governance = parse_usize_arg("--max-governance", &v);
            }
            "--max-lineage" => {
                let Some(v) = args.next() else {
                    eprintln!("--max-lineage requires a value");
                    std::process::exit(2);
                };
                max_lineage = parse_usize_arg("--max-lineage", &v);
            }
            "--decision-index" => {
                let Some(v) = args.next() else {
                    eprintln!("--decision-index requires a value");
                    std::process::exit(2);
                };
                decision_index = Some(PathBuf::from(v));
            }
            "--decision-journal" => {
                let Some(v) = args.next() else {
                    eprintln!("--decision-journal requires a value");
                    std::process::exit(2);
                };
                decision_journal = Some(PathBuf::from(v));
            }
            "--expected-bundle-digest" => {
                let Some(v) = args.next() else {
                    eprintln!("--expected-bundle-digest requires a value");
                    std::process::exit(2);
                };
                expected_bundle_digest = Some(v);
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(input_file) = input else {
        eprintln!("missing --input");
        std::process::exit(2);
    };

    let report = match underlay_devtools::load_pipeline_run_report(&input_file) {
        Ok(report) => report,
        Err(err) => {
            eprintln!("migration report drift failed: {err}");
            std::process::exit(1);
        }
    };

    let thresholds = underlay_migration_core::DriftThresholds {
        max_unresolved_decisions: max_unresolved,
        max_governance_issues: max_governance,
        max_lineage_mismatches: max_lineage,
        require_verify_passed: true,
    };

    let lineage = match (decision_index, decision_journal) {
        (Some(index_path), Some(journal_path)) => {
            let index = match underlay_devtools::load_decision_index(&index_path) {
                Ok(index) => index,
                Err(err) => {
                    eprintln!("migration report drift failed: {err}");
                    std::process::exit(1);
                }
            };
            let journal_records = match underlay_devtools::load_decision_journal(&journal_path) {
                Ok(records) => records,
                Err(err) => {
                    eprintln!("migration report drift failed: {err}");
                    std::process::exit(1);
                }
            };
            Some(underlay_migration_core::DecisionLineageInput {
                index,
                journal_records,
                expected_bundle_digest,
            })
        }
        (None, None) => None,
        _ => {
            eprintln!("--decision-index and --decision-journal must be provided together");
            std::process::exit(2);
        }
    };

    let drift =
        underlay_devtools::build_drift_report_with_lineage(&report, &thresholds, lineage.as_ref());
    println!(
        "drift run_id={} issues={} blocking={}",
        drift.run_id, drift.issue_count, drift.blocking_issue_count
    );
    for line in underlay_devtools::format_drift_category_summary(&drift) {
        println!("category {line}");
    }
    for line in underlay_devtools::format_drift_report(&drift) {
        println!("drift {line}");
    }
}

fn run_migration_report_recovery(mut args: impl Iterator<Item = String>) {
    let mut input: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                let Some(v) = args.next() else {
                    eprintln!("--input requires a value");
                    std::process::exit(2);
                };
                input = Some(PathBuf::from(v));
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(input_file) = input else {
        eprintln!("missing --input");
        std::process::exit(2);
    };

    let report = match underlay_devtools::load_pipeline_run_report(&input_file) {
        Ok(report) => report,
        Err(err) => {
            eprintln!("migration report recovery failed: {err}");
            std::process::exit(1);
        }
    };

    let advisories = underlay_devtools::build_recovery_advisories(&report);
    for line in underlay_devtools::format_recovery_advisories(&advisories) {
        println!("recovery {line}");
    }
}

fn run_migration_report_verify(mut args: impl Iterator<Item = String>) {
    let mut input: Option<PathBuf> = None;
    let mut output_dir: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                let Some(v) = args.next() else {
                    eprintln!("--input requires a value");
                    std::process::exit(2);
                };
                input = Some(PathBuf::from(v));
            }
            "--output-dir" => {
                let Some(v) = args.next() else {
                    eprintln!("--output-dir requires a value");
                    std::process::exit(2);
                };
                output_dir = Some(PathBuf::from(v));
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(input_file) = input else {
        eprintln!("missing --input");
        std::process::exit(2);
    };
    let output_dir = output_dir.unwrap_or_else(|| PathBuf::from(".underlay-migration"));

    let report = match underlay_devtools::load_pipeline_run_report(&input_file) {
        Ok(report) => report,
        Err(err) => {
            eprintln!("migration report verify failed: {err}");
            std::process::exit(1);
        }
    };
    let artifact = match underlay_devtools::build_verification_report(&report) {
        Ok(artifact) => artifact,
        Err(err) => {
            eprintln!("migration report verify failed: {err}");
            std::process::exit(1);
        }
    };
    let written_path = match underlay_devtools::write_verification_artifact(&output_dir, &artifact)
    {
        Ok(path) => path,
        Err(err) => {
            eprintln!("migration report verify failed: {err}");
            std::process::exit(1);
        }
    };

    println!("verify artifact {}", written_path.display());
    for line in underlay_devtools::format_verification_summary(&artifact) {
        println!("verify {line}");
    }
}

fn parse_usize_arg(name: &str, value: &str) -> usize {
    value.parse::<usize>().unwrap_or_else(|_| {
        eprintln!("{name} must be an unsigned integer");
        std::process::exit(2);
    })
}

fn run_migration_run(mut args: impl Iterator<Item = String>) {
    let mut output: Option<PathBuf> = None;
    let mut bundle: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let Some(v) = args.next() else {
                    eprintln!("--output requires a value");
                    std::process::exit(2);
                };
                output = Some(PathBuf::from(v));
            }
            "--bundle" => {
                let Some(v) = args.next() else {
                    eprintln!("--bundle requires a value");
                    std::process::exit(2);
                };
                bundle = Some(v);
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(output_dir) = output else {
        eprintln!("missing --output");
        std::process::exit(2);
    };
    let Some(bundle_ref) = bundle else {
        eprintln!("missing --bundle");
        std::process::exit(2);
    };

    match underlay_devtools::migration_run(&underlay_devtools::BundleRunOptions {
        bundle_ref,
        output_dir,
        local_store_dir: None,
    }) {
        Ok(report) => {
            println!(
                "run {} -> {} ({}, digest={}, run_id={})",
                report.bundle_ref,
                report.output_file.display(),
                report.status,
                report.digest,
                report.run_id
            );
        }
        Err(err) => {
            eprintln!("migration run failed: {err}");
            std::process::exit(1);
        }
    }
}

fn run_bundle_build(mut args: impl Iterator<Item = String>) {
    let mut output: Option<PathBuf> = None;
    let mut source_system: Option<String> = None;
    let mut target_schema_version: Option<String> = None;
    let mut media_dir: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let Some(v) = args.next() else {
                    eprintln!("--output requires a value");
                    std::process::exit(2);
                };
                output = Some(PathBuf::from(v));
            }
            "--source-system" => {
                let Some(v) = args.next() else {
                    eprintln!("--source-system requires a value");
                    std::process::exit(2);
                };
                source_system = Some(v);
            }
            "--target-schema-version" => {
                let Some(v) = args.next() else {
                    eprintln!("--target-schema-version requires a value");
                    std::process::exit(2);
                };
                target_schema_version = Some(v);
            }
            "--media-dir" => {
                let Some(v) = args.next() else {
                    eprintln!("--media-dir requires a value");
                    std::process::exit(2);
                };
                media_dir = Some(PathBuf::from(v));
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(output) = output else {
        eprintln!("missing --output");
        std::process::exit(2);
    };
    let Some(source_system) = source_system else {
        eprintln!("missing --source-system");
        std::process::exit(2);
    };
    let Some(target_schema_version) = target_schema_version else {
        eprintln!("missing --target-schema-version");
        std::process::exit(2);
    };

    match underlay_devtools::migration_bundle_build(&underlay_devtools::BundleBuildOptions {
        output_file: output,
        source_system,
        target_schema_version,
        media_dir,
        media_shard_max_bytes: None,
    }) {
        Ok(report) => {
            println!(
                "bundle written {} (artifact_type={}, layers={}, sidecars={})",
                report.output_file.display(),
                report.artifact_type,
                report.layer_count,
                report.sidecar_count
            );
            println!("bundle digest {}", report.bundle_digest);
            println!(
                "media assets {}, media shards {}",
                report.media_asset_count, report.media_shard_count
            );
        }
        Err(err) => {
            eprintln!("migration bundle build failed: {err}");
            std::process::exit(1);
        }
    }
}

fn run_bundle_publish(mut args: impl Iterator<Item = String>) {
    let mut bundle: Option<PathBuf> = None;
    let mut oci_ref: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bundle" => {
                let Some(v) = args.next() else {
                    eprintln!("--bundle requires a value");
                    std::process::exit(2);
                };
                bundle = Some(PathBuf::from(v));
            }
            "--oci-ref" => {
                let Some(v) = args.next() else {
                    eprintln!("--oci-ref requires a value");
                    std::process::exit(2);
                };
                oci_ref = Some(v);
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(bundle_file) = bundle else {
        eprintln!("missing --bundle");
        std::process::exit(2);
    };
    let Some(oci_ref) = oci_ref else {
        eprintln!("missing --oci-ref");
        std::process::exit(2);
    };

    match underlay_devtools::migration_bundle_publish(&underlay_devtools::BundlePublishOptions {
        bundle_file,
        oci_ref,
        local_store_dir: None,
    }) {
        Ok(report) => {
            println!(
                "publish {} -> {} ({}, digest={})",
                report.bundle_file.display(),
                report.oci_ref,
                report.status,
                report.digest
            );
        }
        Err(err) => {
            eprintln!("migration bundle publish failed: {err}");
            std::process::exit(1);
        }
    }
}

fn run_bundle_pull(mut args: impl Iterator<Item = String>) {
    let mut output: Option<PathBuf> = None;
    let mut oci_ref: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let Some(v) = args.next() else {
                    eprintln!("--output requires a value");
                    std::process::exit(2);
                };
                output = Some(PathBuf::from(v));
            }
            "--oci-ref" => {
                let Some(v) = args.next() else {
                    eprintln!("--oci-ref requires a value");
                    std::process::exit(2);
                };
                oci_ref = Some(v);
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(output_dir) = output else {
        eprintln!("missing --output");
        std::process::exit(2);
    };
    let Some(oci_ref) = oci_ref else {
        eprintln!("missing --oci-ref");
        std::process::exit(2);
    };

    match underlay_devtools::migration_bundle_pull(&underlay_devtools::BundlePullOptions {
        oci_ref,
        output_dir,
        local_store_dir: None,
    }) {
        Ok(report) => {
            println!(
                "pull {} -> {} ({}, digest={})",
                report.oci_ref,
                report.output_file.display(),
                report.status,
                report.digest
            );
        }
        Err(err) => {
            eprintln!("migration bundle pull failed: {err}");
            std::process::exit(1);
        }
    }
}

// ── Seed bundle subcommands ────────────────────────────────────────────

fn run_seed(mut args: impl Iterator<Item = String>) {
    let Some(subcommand) = args.next() else {
        eprintln!("missing seed subcommand");
        print_usage();
        std::process::exit(2);
    };

    if subcommand != "bundle" {
        eprintln!("unknown seed subcommand: {subcommand}");
        print_usage();
        std::process::exit(2);
    }

    let Some(action) = args.next() else {
        eprintln!("missing seed bundle action");
        print_usage();
        std::process::exit(2);
    };

    match action.as_str() {
        "build" => run_seed_bundle_build(args),
        "publish" => run_seed_bundle_publish(args),
        "pull" => run_seed_bundle_pull(args),
        other => {
            eprintln!("unknown seed bundle action: {other}");
            print_usage();
            std::process::exit(2);
        }
    }
}

fn run_seed_bundle_build(mut args: impl Iterator<Item = String>) {
    let mut source: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--source" => {
                let Some(v) = args.next() else {
                    eprintln!("--source requires a value");
                    std::process::exit(2);
                };
                source = Some(PathBuf::from(v));
            }
            "--output" => {
                let Some(v) = args.next() else {
                    eprintln!("--output requires a value");
                    std::process::exit(2);
                };
                output = Some(PathBuf::from(v));
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(source_dir) = source else {
        eprintln!("missing --source");
        std::process::exit(2);
    };
    let Some(output_file) = output else {
        eprintln!("missing --output");
        std::process::exit(2);
    };

    match underlay_devtools::seed_bundle_build(&underlay_devtools::SeedBundleBuildOptions {
        source_dir,
        output_file,
    }) {
        Ok(report) => {
            println!(
                "seed bundle written {} (name={}, artifact_type={}, layers={}, sql_files={}, sql_bytes={})",
                report.output_file.display(),
                report.bundle_name,
                report.artifact_type,
                report.layer_count,
                report.sql_file_count,
                report.total_sql_bytes,
            );
            println!("seed bundle digest {}", report.bundle_digest);
        }
        Err(err) => {
            eprintln!("seed bundle build failed: {err}");
            std::process::exit(1);
        }
    }
}

fn run_seed_bundle_publish(mut args: impl Iterator<Item = String>) {
    let mut bundle: Option<PathBuf> = None;
    let mut oci_ref: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bundle" => {
                let Some(v) = args.next() else {
                    eprintln!("--bundle requires a value");
                    std::process::exit(2);
                };
                bundle = Some(PathBuf::from(v));
            }
            "--oci-ref" => {
                let Some(v) = args.next() else {
                    eprintln!("--oci-ref requires a value");
                    std::process::exit(2);
                };
                oci_ref = Some(v);
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(bundle_file) = bundle else {
        eprintln!("missing --bundle");
        std::process::exit(2);
    };
    let Some(oci_ref) = oci_ref else {
        eprintln!("missing --oci-ref");
        std::process::exit(2);
    };

    match underlay_devtools::seed_bundle_publish(&underlay_devtools::BundlePublishOptions {
        bundle_file,
        oci_ref,
        local_store_dir: None,
    }) {
        Ok(report) => {
            println!(
                "seed publish {} -> {} ({}, digest={})",
                report.bundle_file.display(),
                report.oci_ref,
                report.status,
                report.digest
            );
        }
        Err(err) => {
            eprintln!("seed bundle publish failed: {err}");
            std::process::exit(1);
        }
    }
}

fn run_seed_bundle_pull(mut args: impl Iterator<Item = String>) {
    let mut output: Option<PathBuf> = None;
    let mut oci_ref: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let Some(v) = args.next() else {
                    eprintln!("--output requires a value");
                    std::process::exit(2);
                };
                output = Some(PathBuf::from(v));
            }
            "--oci-ref" => {
                let Some(v) = args.next() else {
                    eprintln!("--oci-ref requires a value");
                    std::process::exit(2);
                };
                oci_ref = Some(v);
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    let Some(output_dir) = output else {
        eprintln!("missing --output");
        std::process::exit(2);
    };
    let Some(oci_ref) = oci_ref else {
        eprintln!("missing --oci-ref");
        std::process::exit(2);
    };

    match underlay_devtools::seed_bundle_pull(&underlay_devtools::SeedBundlePullOptions {
        oci_ref,
        output_dir,
        local_store_dir: None,
    }) {
        Ok(report) => {
            println!(
                "seed pull {} -> {} ({}, digest={}, sql_files={})",
                report.oci_ref,
                report.output_dir.display(),
                report.status,
                report.digest,
                report.sql_file_count,
            );
        }
        Err(err) => {
            eprintln!("seed bundle pull failed: {err}");
            std::process::exit(1);
        }
    }
}
