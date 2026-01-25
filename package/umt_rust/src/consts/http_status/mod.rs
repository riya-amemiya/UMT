//! HTTP Status Codes module.
//!
//! This module provides HTTP status codes organized by category:
//! - 1xx Informational
//! - 2xx Success
//! - 3xx Redirection
//! - 4xx Client Error
//! - 5xx Server Error

pub mod http_client_error_status;
pub mod http_informational_status;
pub mod http_redirection_status;
pub mod http_server_error_status;
pub mod http_success_status;

pub use http_client_error_status::HttpClientErrorStatus;
pub use http_informational_status::HttpInformationalStatus;
pub use http_redirection_status::HttpRedirectionStatus;
pub use http_server_error_status::HttpServerErrorStatus;
pub use http_success_status::HttpSuccessStatus;

/// Combined HTTP status codes struct containing all status code categories.
pub struct HttpStatus;

impl HttpStatus {
    // 1xx Informational
    /// Server has received request headers and client should proceed with request.
    pub const CONTINUE: u16 = http_informational_status::CONTINUE;
    /// Server is switching protocols according to Upgrade header.
    pub const SWITCHING_PROTOCOLS: u16 = http_informational_status::SWITCHING_PROTOCOLS;
    /// Server has received and is processing the request, but no response is available yet.
    pub const PROCESSING: u16 = http_informational_status::PROCESSING;
    /// Server is likely to send a final response with the header fields included in the informational response.
    pub const EARLYHINTS: u16 = http_informational_status::EARLY_HINTS;

    // 2xx Success
    /// Request has succeeded.
    pub const OK: u16 = http_success_status::OK;
    /// Request has succeeded and a new resource has been created.
    pub const CREATED: u16 = http_success_status::CREATED;
    /// Request has been accepted for processing, but the processing has not been completed.
    pub const ACCEPTED: u16 = http_success_status::ACCEPTED;
    /// Server returned transformed information from origin server.
    pub const NON_AUTHORITATIVE_INFORMATION: u16 =
        http_success_status::NON_AUTHORITATIVE_INFORMATION;
    /// Server has fulfilled the request but does not need to return any content.
    pub const NO_CONTENT: u16 = http_success_status::NO_CONTENT;
    /// Server has fulfilled the request and the client should reset the document view.
    pub const RESET_CONTENT: u16 = http_success_status::RESET_CONTENT;
    /// Server has fulfilled the partial GET request for the resource.
    pub const PARTIAL_CONTENT: u16 = http_success_status::PARTIAL_CONTENT;

    // 3xx Redirection
    /// Multiple options for the resource from which the client may choose.
    pub const AMBIGUOUS: u16 = http_redirection_status::AMBIGUOUS;
    /// Resource has been permanently moved to another URI.
    pub const MOVED_PERMANENTLY: u16 = http_redirection_status::MOVED_PERMANENTLY;
    /// Resource temporarily resides under a different URI.
    pub const FOUND: u16 = http_redirection_status::FOUND;
    /// Response to the request can be found under another URI using GET.
    pub const SEE_OTHER: u16 = http_redirection_status::SEE_OTHER;
    /// Resource has not been modified since last requested.
    pub const NOT_MODIFIED: u16 = http_redirection_status::NOT_MODIFIED;
    /// Resource temporarily moved to another URI, using same HTTP method.
    pub const TEMPORARY_REDIRECT: u16 = http_redirection_status::TEMPORARY_REDIRECT;
    /// Resource has been permanently moved to another URI, using same HTTP method.
    pub const PERMANENT_REDIRECT: u16 = http_redirection_status::PERMANENT_REDIRECT;

    // 4xx Client Error
    /// Server cannot or will not process the request due to a client error.
    pub const BAD_REQUEST: u16 = http_client_error_status::BAD_REQUEST;
    /// Request requires user authentication.
    pub const UNAUTHORIZED: u16 = http_client_error_status::UNAUTHORIZED;
    /// Payment is required for access to the resource.
    pub const PAYMENT_REQUIRED: u16 = http_client_error_status::PAYMENT_REQUIRED;
    /// Server understood the request but refuses to authorize it.
    pub const FORBIDDEN: u16 = http_client_error_status::FORBIDDEN;
    /// Server cannot find the requested resource.
    pub const NOT_FOUND: u16 = http_client_error_status::NOT_FOUND;
    /// Method specified in the request is not allowed for the resource.
    pub const METHOD_NOT_ALLOWED: u16 = http_client_error_status::METHOD_NOT_ALLOWED;
    /// Resource cannot generate content according to the Accept headers.
    pub const NOT_ACCEPTABLE: u16 = http_client_error_status::NOT_ACCEPTABLE;
    /// Client must first authenticate itself with the proxy.
    pub const PROXY_AUTHENTICATION_REQUIRED: u16 =
        http_client_error_status::PROXY_AUTHENTICATION_REQUIRED;
    /// Server timed out waiting for the request.
    pub const REQUEST_TIMEOUT: u16 = http_client_error_status::REQUEST_TIMEOUT;
    /// Request conflicts with the current state of the server.
    pub const CONFLICT: u16 = http_client_error_status::CONFLICT;
    /// Resource requested is no longer available and will not be available again.
    pub const GONE: u16 = http_client_error_status::GONE;
    /// Server requires request to have Content-Length header.
    pub const LENGTH_REQUIRED: u16 = http_client_error_status::LENGTH_REQUIRED;
    /// Server does not meet one of the preconditions specified in request headers.
    pub const PRECONDITION_FAILED: u16 = http_client_error_status::PRECONDITION_FAILED;
    /// Request entity is larger than limits defined by server.
    pub const PAYLOAD_TOO_LARGE: u16 = http_client_error_status::PAYLOAD_TOO_LARGE;
    /// URI requested by the client is longer than the server is willing to interpret.
    pub const URI_TOO_LONG: u16 = http_client_error_status::URI_TOO_LONG;
    /// Media format of the requested data is not supported by the server.
    pub const UNSUPPORTED_MEDIA_TYPE: u16 = http_client_error_status::UNSUPPORTED_MEDIA_TYPE;
    /// Range specified by the Range header field cannot be fulfilled.
    pub const REQUESTED_RANGE_NOT_SATISFIABLE: u16 =
        http_client_error_status::REQUESTED_RANGE_NOT_SATISFIABLE;
    /// Expectation indicated by the Expect request header field cannot be met.
    pub const EXPECTATION_FAILED: u16 = http_client_error_status::EXPECTATION_FAILED;
    /// Server refuses to brew coffee because it is a teapot (April Fools' joke).
    pub const I_AM_A_TEAPOT: u16 = http_client_error_status::I_AM_A_TEAPOT;
    /// Request was directed at a server that is not able to produce a response.
    pub const MISDIRECTED: u16 = http_client_error_status::MISDIRECTED;
    /// Request was well-formed but unable to be followed due to semantic errors.
    pub const UNPROCESSABLE_ENTITY: u16 = http_client_error_status::UNPROCESSABLE_ENTITY;
    /// Request failed due to failure of a previous request.
    pub const FAILED_DEPENDENCY: u16 = http_client_error_status::FAILED_DEPENDENCY;
    /// Origin server requires the request to be conditional.
    pub const PRECONDITION_REQUIRED: u16 = http_client_error_status::PRECONDITION_REQUIRED;
    /// User has sent too many requests in a given amount of time.
    pub const TOO_MANY_REQUESTS: u16 = http_client_error_status::TOO_MANY_REQUESTS;

    // 5xx Server Error
    /// Server encountered an unexpected condition that prevented it from fulfilling the request.
    pub const INTERNAL_SERVER_ERROR: u16 = http_server_error_status::INTERNAL_SERVER_ERROR;
    /// Server does not support the functionality required to fulfill the request.
    pub const NOT_IMPLEMENTED: u16 = http_server_error_status::NOT_IMPLEMENTED;
    /// Server received an invalid response from the upstream server.
    pub const BAD_GATEWAY: u16 = http_server_error_status::BAD_GATEWAY;
    /// Server is temporarily unable to handle the request due to being overloaded or down for maintenance.
    pub const SERVICE_UNAVAILABLE: u16 = http_server_error_status::SERVICE_UNAVAILABLE;
    /// Server did not receive a timely response from the upstream server.
    pub const GATEWAY_TIMEOUT: u16 = http_server_error_status::GATEWAY_TIMEOUT;
    /// Server does not support the HTTP protocol version used in the request.
    pub const HTTP_VERSION_NOT_SUPPORTED: u16 =
        http_server_error_status::HTTP_VERSION_NOT_SUPPORTED;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_status_exports() {
        // Verify all status types are accessible
        let _ = HttpClientErrorStatus::BAD_REQUEST;
        let _ = HttpInformationalStatus::CONTINUE;
        let _ = HttpRedirectionStatus::AMBIGUOUS;
        let _ = HttpServerErrorStatus::INTERNAL_SERVER_ERROR;
        let _ = HttpSuccessStatus::OK;
    }

    #[test]
    fn test_http_status_informational() {
        assert_eq!(HttpStatus::CONTINUE, 100);
        assert_eq!(HttpStatus::SWITCHING_PROTOCOLS, 101);
        assert_eq!(HttpStatus::PROCESSING, 102);
        assert_eq!(HttpStatus::EARLYHINTS, 103);
    }

    #[test]
    fn test_http_status_success() {
        assert_eq!(HttpStatus::OK, 200);
        assert_eq!(HttpStatus::CREATED, 201);
        assert_eq!(HttpStatus::ACCEPTED, 202);
        assert_eq!(HttpStatus::NON_AUTHORITATIVE_INFORMATION, 203);
        assert_eq!(HttpStatus::NO_CONTENT, 204);
        assert_eq!(HttpStatus::RESET_CONTENT, 205);
        assert_eq!(HttpStatus::PARTIAL_CONTENT, 206);
    }

    #[test]
    fn test_http_status_redirection() {
        assert_eq!(HttpStatus::AMBIGUOUS, 300);
        assert_eq!(HttpStatus::MOVED_PERMANENTLY, 301);
        assert_eq!(HttpStatus::FOUND, 302);
        assert_eq!(HttpStatus::SEE_OTHER, 303);
        assert_eq!(HttpStatus::NOT_MODIFIED, 304);
        assert_eq!(HttpStatus::TEMPORARY_REDIRECT, 307);
        assert_eq!(HttpStatus::PERMANENT_REDIRECT, 308);
    }

    #[test]
    fn test_http_status_client_error() {
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

    #[test]
    fn test_http_status_server_error() {
        assert_eq!(HttpStatus::INTERNAL_SERVER_ERROR, 500);
        assert_eq!(HttpStatus::NOT_IMPLEMENTED, 501);
        assert_eq!(HttpStatus::BAD_GATEWAY, 502);
        assert_eq!(HttpStatus::SERVICE_UNAVAILABLE, 503);
        assert_eq!(HttpStatus::GATEWAY_TIMEOUT, 504);
        assert_eq!(HttpStatus::HTTP_VERSION_NOT_SUPPORTED, 505);
    }
}
