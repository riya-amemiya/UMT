//! Tests for the HttpRedirectionStatus module.

use umt_rust::consts::HttpRedirectionStatus;

#[test]
fn test_http_redirection_status_codes() {
    assert_eq!(HttpRedirectionStatus::AMBIGUOUS, 300);
    assert_eq!(HttpRedirectionStatus::MOVED_PERMANENTLY, 301);
    assert_eq!(HttpRedirectionStatus::FOUND, 302);
    assert_eq!(HttpRedirectionStatus::SEE_OTHER, 303);
    assert_eq!(HttpRedirectionStatus::NOT_MODIFIED, 304);
    assert_eq!(HttpRedirectionStatus::TEMPORARY_REDIRECT, 307);
    assert_eq!(HttpRedirectionStatus::PERMANENT_REDIRECT, 308);
}
