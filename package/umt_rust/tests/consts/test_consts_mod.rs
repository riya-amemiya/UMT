use umt_rust::consts::*;

#[test]
fn test_clock_constants_accessible() {
    assert_eq!(ONE_SECOND_MS, 1000);
    assert_eq!(ONE_MINUTE_MS, 60_000);
    assert_eq!(ONE_HOUR_MS, 3_600_000);
}

#[test]
fn test_http_status_accessible() {
    assert_eq!(HttpStatus::OK, 200);
    assert_eq!(HttpStatus::NOT_FOUND, 404);
    assert_eq!(HttpStatus::INTERNAL_SERVER_ERROR, 500);
}

#[test]
fn test_individual_http_status_structs_accessible() {
    assert_eq!(HttpInformationalStatus::CONTINUE, 100);
    assert_eq!(HttpSuccessStatus::OK, 200);
    assert_eq!(HttpRedirectionStatus::MOVED_PERMANENTLY, 301);
    assert_eq!(HttpClientErrorStatus::NOT_FOUND, 404);
    assert_eq!(HttpServerErrorStatus::INTERNAL_SERVER_ERROR, 500);
}
