//! Tests for the HttpInformationalStatus module.

use umt_rust::consts::HttpInformationalStatus;

#[test]
fn test_http_informational_status_codes() {
    assert_eq!(HttpInformationalStatus::CONTINUE, 100);
    assert_eq!(HttpInformationalStatus::SWITCHING_PROTOCOLS, 101);
    assert_eq!(HttpInformationalStatus::PROCESSING, 102);
    assert_eq!(HttpInformationalStatus::EARLYHINTS, 103);
}

use umt_rust::consts::http_status::http_informational_status::*;
use umt_rust::consts::http_status::*;

#[test]
fn test_informational_status_codes() {
    assert_eq!(HttpInformationalStatus::CONTINUE, 100);
    assert_eq!(HttpInformationalStatus::SWITCHING_PROTOCOLS, 101);
    assert_eq!(HttpInformationalStatus::PROCESSING, 102);
    assert_eq!(HttpInformationalStatus::EARLYHINTS, 103);
}

#[test]
fn test_informational_status_constants() {
    assert_eq!(CONTINUE, 100);
    assert_eq!(SWITCHING_PROTOCOLS, 101);
    assert_eq!(PROCESSING, 102);
    assert_eq!(EARLY_HINTS, 103);
}
