//! HTTP 1xx Informational Status Codes.
//!
//! Indicates a provisional response, consisting only of the Status-Line and optional headers,
//! and is terminated by an empty line.

/// Server has received request headers and client should proceed with request.
pub const CONTINUE: u16 = 100;

/// Server is switching protocols according to Upgrade header.
pub const SWITCHING_PROTOCOLS: u16 = 101;

/// Server has received and is processing the request, but no response is available yet.
pub const PROCESSING: u16 = 102;

/// Server is likely to send a final response with the header fields included in the informational response.
pub const EARLY_HINTS: u16 = 103;

/// HTTP 1xx Informational status codes as a module-like struct.
pub struct HttpInformationalStatus;

impl HttpInformationalStatus {
    /// Server has received request headers and client should proceed with request.
    pub const CONTINUE: u16 = CONTINUE;

    /// Server is switching protocols according to Upgrade header.
    pub const SWITCHING_PROTOCOLS: u16 = SWITCHING_PROTOCOLS;

    /// Server has received and is processing the request, but no response is available yet.
    pub const PROCESSING: u16 = PROCESSING;

    /// Server is likely to send a final response with the header fields included in the informational response.
    pub const EARLYHINTS: u16 = EARLY_HINTS;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_informational_status_codes() {
        assert_eq!(HttpInformationalStatus::CONTINUE, 100);
        assert_eq!(HttpInformationalStatus::SWITCHING_PROTOCOLS, 101);
        assert_eq!(HttpInformationalStatus::PROCESSING, 102);
        assert_eq!(HttpInformationalStatus::EARLYHINTS, 103);
    }

    #[test]
    fn test_informational_status_constants() {
        assert_eq!(CONTINUE, 100);
        assert_eq!(SWITCHING_PROTOCOLS, 101);
        assert_eq!(PROCESSING, 102);
        assert_eq!(EARLY_HINTS, 103);
    }
}
