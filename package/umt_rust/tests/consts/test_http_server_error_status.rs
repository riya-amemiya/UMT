//! Tests for the HttpServerErrorStatus module.

use umt_rust::consts::HttpServerErrorStatus;

#[test]
fn test_http_server_error_status_codes() {
    assert_eq!(HttpServerErrorStatus::INTERNAL_SERVER_ERROR, 500);
    assert_eq!(HttpServerErrorStatus::NOT_IMPLEMENTED, 501);
    assert_eq!(HttpServerErrorStatus::BAD_GATEWAY, 502);
    assert_eq!(HttpServerErrorStatus::SERVICE_UNAVAILABLE, 503);
    assert_eq!(HttpServerErrorStatus::GATEWAY_TIMEOUT, 504);
    assert_eq!(HttpServerErrorStatus::HTTP_VERSION_NOT_SUPPORTED, 505);
}
