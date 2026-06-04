use std::cell::Cell;
use umt_rust::async_util::{Backoff, RetryAsyncOptions, umt_retry};

fn options_with<S>(
    retries: u32,
    delay: u64,
    backoff: Backoff,
    jitter: bool,
    should_retry: S,
) -> RetryAsyncOptions<S, fn(&String, u32)>
where
    S: Fn(&String, u32) -> bool,
{
    RetryAsyncOptions {
        retries,
        delay,
        backoff,
        jitter,
        should_retry,
        on_retry: None,
    }
}

#[test]
fn test_retry_returns_result_on_first_success() {
    let calls = Cell::new(0);
    let result = umt_retry(
        || {
            calls.set(calls.get() + 1);
            Ok::<&str, String>("ok")
        },
        RetryAsyncOptions::default(),
    );
    assert_eq!(result, Ok("ok"));
    assert_eq!(calls.get(), 1);
}

#[test]
fn test_retry_retries_then_resolves() {
    let attempts = Cell::new(0);
    let result = umt_retry(
        || {
            attempts.set(attempts.get() + 1);
            if attempts.get() < 3 {
                Err("nope".to_string())
            } else {
                Ok("ok")
            }
        },
        options_with(3, 1, Backoff::Fixed, false, |_, _| true),
    );
    assert_eq!(result, Ok("ok"));
    assert_eq!(attempts.get(), 3);
}

#[test]
fn test_retry_rejects_after_exhausting_retries() {
    let calls = Cell::new(0);
    let result: Result<&str, String> = umt_retry(
        || {
            calls.set(calls.get() + 1);
            Err("nope".to_string())
        },
        options_with(2, 1, Backoff::Fixed, false, |_, _| true),
    );
    assert_eq!(result, Err("nope".to_string()));
    assert_eq!(calls.get(), 3);
}

#[test]
fn test_retry_respects_should_retry_false() {
    let calls = Cell::new(0);
    let result: Result<&str, String> = umt_retry(
        || {
            calls.set(calls.get() + 1);
            Err("fatal".to_string())
        },
        options_with(5, 1, Backoff::Fixed, false, |_, _| false),
    );
    assert_eq!(result, Err("fatal".to_string()));
    assert_eq!(calls.get(), 1);
}

#[test]
fn test_retry_invokes_on_retry_before_each_retry() {
    let attempts = Cell::new(0);
    let on_retry_calls = Cell::new(0);
    let result = umt_retry(
        || {
            attempts.set(attempts.get() + 1);
            if attempts.get() < 2 {
                Err("nope".to_string())
            } else {
                Ok("ok")
            }
        },
        RetryAsyncOptions {
            retries: 2,
            delay: 1,
            backoff: Backoff::Fixed,
            jitter: false,
            should_retry: |_: &String, _| true,
            on_retry: Some(|_: &String, _| {
                on_retry_calls.set(on_retry_calls.get() + 1);
            }),
        },
    );
    assert_eq!(result, Ok("ok"));
    assert_eq!(on_retry_calls.get(), 1);
}

#[test]
fn test_retry_supports_linear_backoff() {
    let attempts = Cell::new(0);
    let _ = umt_retry(
        || {
            attempts.set(attempts.get() + 1);
            if attempts.get() < 2 {
                Err("nope".to_string())
            } else {
                Ok("ok")
            }
        },
        options_with(2, 1, Backoff::Linear, false, |_, _| true),
    );
    assert_eq!(attempts.get(), 2);
}

#[test]
fn test_retry_supports_exponential_backoff() {
    let attempts = Cell::new(0);
    let _ = umt_retry(
        || {
            attempts.set(attempts.get() + 1);
            if attempts.get() < 2 {
                Err("nope".to_string())
            } else {
                Ok("ok")
            }
        },
        options_with(2, 1, Backoff::Exponential, false, |_, _| true),
    );
    assert_eq!(attempts.get(), 2);
}

#[test]
fn test_retry_supports_jitter() {
    let attempts = Cell::new(0);
    let _ = umt_retry(
        || {
            attempts.set(attempts.get() + 1);
            if attempts.get() < 2 {
                Err("nope".to_string())
            } else {
                Ok("ok")
            }
        },
        options_with(2, 1, Backoff::Fixed, true, |_, _| true),
    );
    assert_eq!(attempts.get(), 2);
}
