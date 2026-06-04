use std::time::Instant;
use umt_rust::async_util::umt_sleep;

#[test]
fn test_sleep_blocks_for_at_least_the_duration() {
    let start = Instant::now();
    umt_sleep(20);
    assert!(start.elapsed().as_millis() >= 20);
}

#[test]
fn test_sleep_returns_immediately_for_zero() {
    umt_sleep(0);
}
