use std::time::Duration;

/// Retry backoff strategy.
#[derive(Debug, Clone)]
pub enum BackoffStrategy {
    /// No delay between retries
    None,
    /// Fixed delay between retries
    Fixed(Duration),
    /// Exponential backoff: min(base * 2^attempt, max), with optional deterministic jitter.
    Exponential {
        base: Duration,
        max: Duration,
        jitter: Option<BackoffJitter>,
    },
}

/// Jitter settings for exponential backoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackoffJitter {
    /// Maximum extra percentage to add to the base delay.
    pub max_percent: u8,
}

impl Default for BackoffJitter {
    fn default() -> Self {
        Self { max_percent: 30 }
    }
}

impl BackoffJitter {
    fn extra_delay(&self, base_delay: Duration, seed: u64) -> Duration {
        if self.max_percent == 0 || base_delay.is_zero() {
            return Duration::ZERO;
        }

        let max_extra_millis =
            ((base_delay.as_millis() * self.max_percent as u128) / 100).max(1_u128);
        let mixed = splitmix64(seed);
        let extra_millis = (mixed as u128) % (max_extra_millis + 1);
        Duration::from_millis(extra_millis as u64)
    }
}

impl BackoffStrategy {
    /// Calculate the delay for a given attempt number (0-indexed).
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        self.delay_for_attempt_with_seed(attempt, 0)
    }

    /// Calculate the delay for a given attempt number (0-indexed) with a deterministic seed.
    pub fn delay_for_attempt_with_seed(&self, attempt: u32, seed: u64) -> Duration {
        match self {
            BackoffStrategy::None => Duration::ZERO,
            BackoffStrategy::Fixed(d) => *d,
            BackoffStrategy::Exponential { base, max, jitter } => {
                let multiplier = 2u64.saturating_pow(attempt);
                let base_delay = base.saturating_mul(multiplier as u32);
                let capped = std::cmp::min(base_delay, *max);
                match jitter {
                    Some(jitter) => std::cmp::min(
                        capped.saturating_add(jitter.extra_delay(capped, seed)),
                        *max,
                    ),
                    None => capped,
                }
            }
        }
    }
}

fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = value;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
