// Retry/backoff helper for flaky RPC calls against the Soroban network.
use std::thread::sleep;
use std::time::Duration;

pub struct RetryOptions {
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
}

impl Default for RetryOptions {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 200,
            max_delay_ms: 5_000,
        }
    }
}

/// Retry a fallible operation with exponential backoff, capped at `max_delay_ms`.
pub fn with_retry<T, E>(
    options: &RetryOptions,
    mut op: impl FnMut() -> Result<T, E>,
) -> Result<T, E> {
    let mut attempt = 0;
    loop {
        match op() {
            Ok(value) => return Ok(value),
            Err(err) => {
                if attempt >= options.max_retries {
                    return Err(err);
                }
                let delay = options
                    .base_delay_ms
                    .saturating_mul(1u64 << attempt)
                    .min(options.max_delay_ms);
                sleep(Duration::from_millis(delay));
                attempt += 1;
            }
        }
    }
}
