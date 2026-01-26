//! HTTP 5xx Server Error Status Codes.
//!
//! Indicates that the server failed to fulfill a valid request.

/// Server encountered an unexpected condition that prevented it from fulfilling the request.
pub const INTERNAL_SERVER_ERROR: u16 = 500;

/// Server does not support the functionality required to fulfill the request.
pub const NOT_IMPLEMENTED: u16 = 501;

/// Server received an invalid response from the upstream server.
pub const BAD_GATEWAY: u16 = 502;

/// Server is temporarily unable to handle the request due to being overloaded or down for maintenance.
pub const SERVICE_UNAVAILABLE: u16 = 503;

/// Server did not receive a timely response from the upstream server.
pub const GATEWAY_TIMEOUT: u16 = 504;

/// Server does not support the HTTP protocol version used in the request.
pub const HTTP_VERSION_NOT_SUPPORTED: u16 = 505;

/// HTTP 5xx Server Error status codes as a module-like struct.
pub struct HttpServerErrorStatus;

impl HttpServerErrorStatus {
    /// Server encountered an unexpected condition that prevented it from fulfilling the request.
    pub const INTERNAL_SERVER_ERROR: u16 = INTERNAL_SERVER_ERROR;

    /// Server does not support the functionality required to fulfill the request.
    pub const NOT_IMPLEMENTED: u16 = NOT_IMPLEMENTED;

    /// Server received an invalid response from the upstream server.
    pub const BAD_GATEWAY: u16 = BAD_GATEWAY;

    /// Server is temporarily unable to handle the request due to being overloaded or down for maintenance.
    pub const SERVICE_UNAVAILABLE: u16 = SERVICE_UNAVAILABLE;

    /// Server did not receive a timely response from the upstream server.
    pub const GATEWAY_TIMEOUT: u16 = GATEWAY_TIMEOUT;

    /// Server does not support the HTTP protocol version used in the request.
    pub const HTTP_VERSION_NOT_SUPPORTED: u16 = HTTP_VERSION_NOT_SUPPORTED;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_error_status_codes() {
        assert_eq!(HttpServerErrorStatus::INTERNAL_SERVER_ERROR, 500);
        assert_eq!(HttpServerErrorStatus::NOT_IMPLEMENTED, 501);
        assert_eq!(HttpServerErrorStatus::BAD_GATEWAY, 502);
        assert_eq!(HttpServerErrorStatus::SERVICE_UNAVAILABLE, 503);
        assert_eq!(HttpServerErrorStatus::GATEWAY_TIMEOUT, 504);
        assert_eq!(HttpServerErrorStatus::HTTP_VERSION_NOT_SUPPORTED, 505);
    }

    #[test]
    fn test_server_error_status_constants() {
        assert_eq!(INTERNAL_SERVER_ERROR, 500);
        assert_eq!(NOT_IMPLEMENTED, 501);
        assert_eq!(BAD_GATEWAY, 502);
        assert_eq!(SERVICE_UNAVAILABLE, 503);
        assert_eq!(GATEWAY_TIMEOUT, 504);
        assert_eq!(HTTP_VERSION_NOT_SUPPORTED, 505);
    }
}
