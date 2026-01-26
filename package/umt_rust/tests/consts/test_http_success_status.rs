//! Tests for the HttpSuccessStatus module.

use umt_rust::consts::HttpSuccessStatus;

#[test]
fn test_http_success_status_codes() {
    assert_eq!(HttpSuccessStatus::OK, 200);
    assert_eq!(HttpSuccessStatus::CREATED, 201);
    assert_eq!(HttpSuccessStatus::ACCEPTED, 202);
    assert_eq!(HttpSuccessStatus::NON_AUTHORITATIVE_INFORMATION, 203);
    assert_eq!(HttpSuccessStatus::NO_CONTENT, 204);
    assert_eq!(HttpSuccessStatus::RESET_CONTENT, 205);
    assert_eq!(HttpSuccessStatus::PARTIAL_CONTENT, 206);
}

use umt_rust::consts::http_status::http_success_status::*;
use umt_rust::consts::http_status::*;

#[test]
fn test_success_status_codes() {
    assert_eq!(HttpSuccessStatus::OK, 200);
    assert_eq!(HttpSuccessStatus::CREATED, 201);
    assert_eq!(HttpSuccessStatus::ACCEPTED, 202);
    assert_eq!(HttpSuccessStatus::NON_AUTHORITATIVE_INFORMATION, 203);
    assert_eq!(HttpSuccessStatus::NO_CONTENT, 204);
    assert_eq!(HttpSuccessStatus::RESET_CONTENT, 205);
    assert_eq!(HttpSuccessStatus::PARTIAL_CONTENT, 206);
}

#[test]
fn test_success_status_constants() {
    assert_eq!(OK, 200);
    assert_eq!(CREATED, 201);
    assert_eq!(ACCEPTED, 202);
    assert_eq!(NON_AUTHORITATIVE_INFORMATION, 203);
    assert_eq!(NO_CONTENT, 204);
    assert_eq!(RESET_CONTENT, 205);
    assert_eq!(PARTIAL_CONTENT, 206);
}
