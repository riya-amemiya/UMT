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

use umt_rust::consts::http_status::http_redirection_status::*;
use umt_rust::consts::http_status::*;

#[test]
fn test_redirection_status_codes() {
    assert_eq!(HttpRedirectionStatus::AMBIGUOUS, 300);
    assert_eq!(HttpRedirectionStatus::MOVED_PERMANENTLY, 301);
    assert_eq!(HttpRedirectionStatus::FOUND, 302);
    assert_eq!(HttpRedirectionStatus::SEE_OTHER, 303);
    assert_eq!(HttpRedirectionStatus::NOT_MODIFIED, 304);
    assert_eq!(HttpRedirectionStatus::TEMPORARY_REDIRECT, 307);
    assert_eq!(HttpRedirectionStatus::PERMANENT_REDIRECT, 308);
}

#[test]
fn test_redirection_status_constants() {
    assert_eq!(AMBIGUOUS, 300);
    assert_eq!(MOVED_PERMANENTLY, 301);
    assert_eq!(FOUND, 302);
    assert_eq!(SEE_OTHER, 303);
    assert_eq!(NOT_MODIFIED, 304);
    assert_eq!(TEMPORARY_REDIRECT, 307);
    assert_eq!(PERMANENT_REDIRECT, 308);
}
