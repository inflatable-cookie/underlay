use std::path::PathBuf;

use crate::cli::{parse_usize_arg, print_usage};

pub(crate) fn run(mut args: impl Iterator<Item = String>) {
    let Some(report_type) = args.next() else {
        eprintln!("missing migration report type");
        print_usage();
        std::process::exit(2);
    };

    match report_type.as_str() {
        "governance" => governance(args),
        "policy" => policy(args),
        "drift" => drift(args),
        "recovery" => recovery(args),
        "verify" => verify(args),
        "integrity" => integrity(args),
        "audit" => audit(args),
        _ => {
            eprintln!("unknown migration report type: {report_type}");
            print_usage();
            std::process::exit(2);
        }
    }
}

fn policy(mut args: impl Iterator<Item = String>) {
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

fn integrity(mut args: impl Iterator<Item = String>) {
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

fn audit(mut args: impl Iterator<Item = String>) {
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

fn governance(mut args: impl Iterator<Item = String>) {
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

fn drift(mut args: impl Iterator<Item = String>) {
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

fn recovery(mut args: impl Iterator<Item = String>) {
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

fn verify(mut args: impl Iterator<Item = String>) {
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
