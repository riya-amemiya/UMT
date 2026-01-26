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

use umt_rust::consts::http_status::http_client_error_status::*;
use umt_rust::consts::http_status::*;

#[test]
fn test_client_error_status_codes() {
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

#[test]
fn test_client_error_status_constants() {
    assert_eq!(BAD_REQUEST, 400);
    assert_eq!(UNAUTHORIZED, 401);
    assert_eq!(PAYMENT_REQUIRED, 402);
    assert_eq!(FORBIDDEN, 403);
    assert_eq!(NOT_FOUND, 404);
    assert_eq!(METHOD_NOT_ALLOWED, 405);
    assert_eq!(NOT_ACCEPTABLE, 406);
    assert_eq!(PROXY_AUTHENTICATION_REQUIRED, 407);
    assert_eq!(REQUEST_TIMEOUT, 408);
    assert_eq!(CONFLICT, 409);
    assert_eq!(GONE, 410);
    assert_eq!(LENGTH_REQUIRED, 411);
    assert_eq!(PRECONDITION_FAILED, 412);
    assert_eq!(PAYLOAD_TOO_LARGE, 413);
    assert_eq!(URI_TOO_LONG, 414);
    assert_eq!(UNSUPPORTED_MEDIA_TYPE, 415);
    assert_eq!(REQUESTED_RANGE_NOT_SATISFIABLE, 416);
    assert_eq!(EXPECTATION_FAILED, 417);
    assert_eq!(I_AM_A_TEAPOT, 418);
    assert_eq!(MISDIRECTED, 421);
    assert_eq!(UNPROCESSABLE_ENTITY, 422);
    assert_eq!(FAILED_DEPENDENCY, 424);
    assert_eq!(PRECONDITION_REQUIRED, 428);
    assert_eq!(TOO_MANY_REQUESTS, 429);
}
