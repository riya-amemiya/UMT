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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_status_codes() {
        assert_eq!(HttpSuccessStatus::OK, 200);
        assert_eq!(HttpSuccessStatus::CREATED, 201);
        assert_eq!(HttpSuccessStatus::ACCEPTED, 202);
        assert_eq!(HttpSuccessStatus::NON_AUTHORITATIVE_INFORMATION, 203);
        assert_eq!(HttpSuccessStatus::NO_CONTENT, 204);
        assert_eq!(HttpSuccessStatus::RESET_CONTENT, 205);
        assert_eq!(HttpSuccessStatus::PARTIAL_CONTENT, 206);
    }

    #[test]
    fn test_success_status_constants() {
        assert_eq!(OK, 200);
        assert_eq!(CREATED, 201);
        assert_eq!(ACCEPTED, 202);
        assert_eq!(NON_AUTHORITATIVE_INFORMATION, 203);
        assert_eq!(NO_CONTENT, 204);
        assert_eq!(RESET_CONTENT, 205);
        assert_eq!(PARTIAL_CONTENT, 206);
    }
}
