//! Tests for the HttpStatus module (combined status codes).

use umt_rust::consts::{
    HttpClientErrorStatus, HttpInformationalStatus, HttpRedirectionStatus, HttpServerErrorStatus,
    HttpStatus, HttpSuccessStatus,
};

mod exports {
    use super::*;

    #[test]
    fn test_all_http_status_types_are_exported() {
        // Verify all status types are accessible
        let _ = HttpClientErrorStatus::BAD_REQUEST;
        let _ = HttpInformationalStatus::CONTINUE;
        let _ = HttpRedirectionStatus::AMBIGUOUS;
        let _ = HttpServerErrorStatus::INTERNAL_SERVER_ERROR;
        let _ = HttpSuccessStatus::OK;
    }
}

mod informational_1xx {
    use super::*;

    #[test]
    fn test_informational_status_codes() {
        assert_eq!(HttpStatus::CONTINUE, 100);
        assert_eq!(HttpStatus::SWITCHING_PROTOCOLS, 101);
        assert_eq!(HttpStatus::PROCESSING, 102);
        assert_eq!(HttpStatus::EARLYHINTS, 103);
    }
}

mod success_2xx {
    use super::*;

    #[test]
    fn test_success_status_codes() {
        assert_eq!(HttpStatus::OK, 200);
        assert_eq!(HttpStatus::CREATED, 201);
        assert_eq!(HttpStatus::ACCEPTED, 202);
        assert_eq!(HttpStatus::NON_AUTHORITATIVE_INFORMATION, 203);
        assert_eq!(HttpStatus::NO_CONTENT, 204);
        assert_eq!(HttpStatus::RESET_CONTENT, 205);
        assert_eq!(HttpStatus::PARTIAL_CONTENT, 206);
    }
}

mod redirection_3xx {
    use super::*;

    #[test]
    fn test_redirection_status_codes() {
        assert_eq!(HttpStatus::AMBIGUOUS, 300);
        assert_eq!(HttpStatus::MOVED_PERMANENTLY, 301);
        assert_eq!(HttpStatus::FOUND, 302);
        assert_eq!(HttpStatus::SEE_OTHER, 303);
        assert_eq!(HttpStatus::NOT_MODIFIED, 304);
        assert_eq!(HttpStatus::TEMPORARY_REDIRECT, 307);
        assert_eq!(HttpStatus::PERMANENT_REDIRECT, 308);
    }
}

mod client_error_4xx {
    use super::*;

    #[test]
    fn test_client_error_status_codes() {
        assert_eq!(HttpStatus::BAD_REQUEST, 400);
        assert_eq!(HttpStatus::UNAUTHORIZED, 401);
        assert_eq!(HttpStatus::PAYMENT_REQUIRED, 402);
        assert_eq!(HttpStatus::FORBIDDEN, 403);
        assert_eq!(HttpStatus::NOT_FOUND, 404);
        assert_eq!(HttpStatus::METHOD_NOT_ALLOWED, 405);
        assert_eq!(HttpStatus::NOT_ACCEPTABLE, 406);
        assert_eq!(HttpStatus::PROXY_AUTHENTICATION_REQUIRED, 407);
        assert_eq!(HttpStatus::REQUEST_TIMEOUT, 408);
        assert_eq!(HttpStatus::CONFLICT, 409);
        assert_eq!(HttpStatus::GONE, 410);
        assert_eq!(HttpStatus::LENGTH_REQUIRED, 411);
        assert_eq!(HttpStatus::PRECONDITION_FAILED, 412);
        assert_eq!(HttpStatus::PAYLOAD_TOO_LARGE, 413);
        assert_eq!(HttpStatus::URI_TOO_LONG, 414);
        assert_eq!(HttpStatus::UNSUPPORTED_MEDIA_TYPE, 415);
        assert_eq!(HttpStatus::REQUESTED_RANGE_NOT_SATISFIABLE, 416);
        assert_eq!(HttpStatus::EXPECTATION_FAILED, 417);
        assert_eq!(HttpStatus::I_AM_A_TEAPOT, 418);
        assert_eq!(HttpStatus::MISDIRECTED, 421);
        assert_eq!(HttpStatus::UNPROCESSABLE_ENTITY, 422);
        assert_eq!(HttpStatus::FAILED_DEPENDENCY, 424);
        assert_eq!(HttpStatus::PRECONDITION_REQUIRED, 428);
        assert_eq!(HttpStatus::TOO_MANY_REQUESTS, 429);
    }
}

mod server_error_5xx {
    use super::*;

    #[test]
    fn test_server_error_status_codes() {
        assert_eq!(HttpStatus::INTERNAL_SERVER_ERROR, 500);
        assert_eq!(HttpStatus::NOT_IMPLEMENTED, 501);
        assert_eq!(HttpStatus::BAD_GATEWAY, 502);
        assert_eq!(HttpStatus::SERVICE_UNAVAILABLE, 503);
        assert_eq!(HttpStatus::GATEWAY_TIMEOUT, 504);
        assert_eq!(HttpStatus::HTTP_VERSION_NOT_SUPPORTED, 505);
    }
}
