//! HTTP 2xx Success Status Codes.
//!
//! Indicates that the client's request was successfully received, understood, and accepted.

/// Request has succeeded.
pub const OK: u16 = 200;

/// Request has succeeded and a new resource has been created.
pub const CREATED: u16 = 201;

/// Request has been accepted for processing, but the processing has not been completed.
pub const ACCEPTED: u16 = 202;

/// Server returned transformed information from origin server.
pub const NON_AUTHORITATIVE_INFORMATION: u16 = 203;

/// Server has fulfilled the request but does not need to return any content.
pub const NO_CONTENT: u16 = 204;

/// Server has fulfilled the request and the client should reset the document view.
pub const RESET_CONTENT: u16 = 205;

/// Server has fulfilled the partial GET request for the resource.
pub const PARTIAL_CONTENT: u16 = 206;

/// HTTP 2xx Success status codes as a module-like struct.
pub struct HttpSuccessStatus;

impl HttpSuccessStatus {
    /// Request has succeeded.
    pub const OK: u16 = OK;

    /// Request has succeeded and a new resource has been created.
    pub const CREATED: u16 = CREATED;

    /// Request has been accepted for processing, but the processing has not been completed.
    pub const ACCEPTED: u16 = ACCEPTED;

    /// Server returned transformed information from origin server.
    pub const NON_AUTHORITATIVE_INFORMATION: u16 = NON_AUTHORITATIVE_INFORMATION;

    /// Server has fulfilled the request but does not need to return any content.
    pub const NO_CONTENT: u16 = NO_CONTENT;

    /// Server has fulfilled the request and the client should reset the document view.
    pub const RESET_CONTENT: u16 = RESET_CONTENT;

    /// Server has fulfilled the partial GET request for the resource.
    pub const PARTIAL_CONTENT: u16 = PARTIAL_CONTENT;
}
