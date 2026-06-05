use std::time::Instant;

use crate::context::MigrationContext;
use crate::errors::MigrationResult;
use crate::plugin::MigrationPlugin;
use crate::verification::VerificationInput;

use super::{evaluate_verification_rules, VerificationBenchmarkResult};

pub async fn benchmark_verification_paths<P>(
    plugin: &P,
    ctx: &MigrationContext,
    input: &VerificationInput,
    iterations: usize,
) -> MigrationResult<VerificationBenchmarkResult>
where
    P: MigrationPlugin,
{
    let iterations = iterations.max(1);

    let declarative_start = Instant::now();
    for _ in 0..iterations {
        let _ = evaluate_verification_rules(input);
    }
    let declarative_elapsed = declarative_start.elapsed();

    let plugin_start = Instant::now();
    for _ in 0..iterations {
        let _ = plugin
            .verify_semantics(ctx, input)
            .await
            .map_err(|err| crate::errors::MigrationError::Plugin(err.to_string()))?;
    }
    let plugin_elapsed = plugin_start.elapsed();

    Ok(VerificationBenchmarkResult {
        iterations,
        declarative_elapsed,
        plugin_elapsed,
        declarative_avg_micros: declarative_elapsed.as_micros() / iterations as u128,
        plugin_avg_micros: plugin_elapsed.as_micros() / iterations as u128,
    })
}
