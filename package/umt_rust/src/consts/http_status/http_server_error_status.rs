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
