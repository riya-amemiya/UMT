//! Tests for the HttpClientErrorStatus module.

use umt_rust::consts::HttpClientErrorStatus;

#[test]
fn test_http_client_error_status_codes() {
    assert_eq!(HttpClientErrorStatus::BAD_REQUEST, 400);
    assert_eq!(HttpClientErrorStatus::UNAUTHORIZED, 401);
    assert_eq!(HttpClientErrorStatus::PAYMENT_REQUIRED, 402);
    assert_eq!(HttpClientErrorStatus::FORBIDDEN, 403);
    assert_eq!(HttpClientErrorStatus::NOT_FOUND, 404);
    assert_eq!(HttpClientErrorStatus::METHOD_NOT_ALLOWED, 405);
    assert_eq!(HttpClientErrorStatus::NOT_ACCEPTABLE, 406);
    assert_eq!(HttpClientErrorStatus::PROXY_AUTHENTICATION_REQUIRED, 407);
    assert_eq!(HttpClientErrorStatus::REQUEST_TIMEOUT, 408);
    assert_eq!(HttpClientErrorStatus::CONFLICT, 409);
    assert_eq!(HttpClientErrorStatus::GONE, 410);
    assert_eq!(HttpClientErrorStatus::LENGTH_REQUIRED, 411);
    assert_eq!(HttpClientErrorStatus::PRECONDITION_FAILED, 412);
    assert_eq!(HttpClientErrorStatus::PAYLOAD_TOO_LARGE, 413);
    assert_eq!(HttpClientErrorStatus::URI_TOO_LONG, 414);
    assert_eq!(HttpClientErrorStatus::UNSUPPORTED_MEDIA_TYPE, 415);
    assert_eq!(HttpClientErrorStatus::REQUESTED_RANGE_NOT_SATISFIABLE, 416);
    assert_eq!(HttpClientErrorStatus::EXPECTATION_FAILED, 417);
    assert_eq!(HttpClientErrorStatus::I_AM_A_TEAPOT, 418);
    assert_eq!(HttpClientErrorStatus::MISDIRECTED, 421);
    assert_eq!(HttpClientErrorStatus::UNPROCESSABLE_ENTITY, 422);
    assert_eq!(HttpClientErrorStatus::FAILED_DEPENDENCY, 424);
    assert_eq!(HttpClientErrorStatus::PRECONDITION_REQUIRED, 428);
    assert_eq!(HttpClientErrorStatus::TOO_MANY_REQUESTS, 429);
}
