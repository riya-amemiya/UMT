//! HTTP 3xx Redirection Status Codes.
//!
//! Indicates that further action needs to be taken by the user agent in order to fulfill the request.

/// Multiple options for the resource from which the client may choose.
pub const AMBIGUOUS: u16 = 300;

/// Resource has been permanently moved to another URI.
pub const MOVED_PERMANENTLY: u16 = 301;

/// Resource temporarily resides under a different URI.
pub const FOUND: u16 = 302;

/// Response to the request can be found under another URI using GET.
pub const SEE_OTHER: u16 = 303;

/// Resource has not been modified since last requested.
pub const NOT_MODIFIED: u16 = 304;

/// Resource temporarily moved to another URI, using same HTTP method.
pub const TEMPORARY_REDIRECT: u16 = 307;

/// Resource has been permanently moved to another URI, using same HTTP method.
pub const PERMANENT_REDIRECT: u16 = 308;

/// HTTP 3xx Redirection status codes as a module-like struct.
pub struct HttpRedirectionStatus;

impl HttpRedirectionStatus {
    /// Multiple options for the resource from which the client may choose.
    pub const AMBIGUOUS: u16 = AMBIGUOUS;

    /// Resource has been permanently moved to another URI.
    pub const MOVED_PERMANENTLY: u16 = MOVED_PERMANENTLY;

    /// Resource temporarily resides under a different URI.
    pub const FOUND: u16 = FOUND;

    /// Response to the request can be found under another URI using GET.
    pub const SEE_OTHER: u16 = SEE_OTHER;

    /// Resource has not been modified since last requested.
    pub const NOT_MODIFIED: u16 = NOT_MODIFIED;

    /// Resource temporarily moved to another URI, using same HTTP method.
    pub const TEMPORARY_REDIRECT: u16 = TEMPORARY_REDIRECT;

    /// Resource has been permanently moved to another URI, using same HTTP method.
    pub const PERMANENT_REDIRECT: u16 = PERMANENT_REDIRECT;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redirection_status_codes() {
        assert_eq!(HttpRedirectionStatus::AMBIGUOUS, 300);
        assert_eq!(HttpRedirectionStatus::MOVED_PERMANENTLY, 301);
        assert_eq!(HttpRedirectionStatus::FOUND, 302);
        assert_eq!(HttpRedirectionStatus::SEE_OTHER, 303);
        assert_eq!(HttpRedirectionStatus::NOT_MODIFIED, 304);
        assert_eq!(HttpRedirectionStatus::TEMPORARY_REDIRECT, 307);
        assert_eq!(HttpRedirectionStatus::PERMANENT_REDIRECT, 308);
    }

    #[test]
    fn test_redirection_status_constants() {
        assert_eq!(AMBIGUOUS, 300);
        assert_eq!(MOVED_PERMANENTLY, 301);
        assert_eq!(FOUND, 302);
        assert_eq!(SEE_OTHER, 303);
        assert_eq!(NOT_MODIFIED, 304);
        assert_eq!(TEMPORARY_REDIRECT, 307);
        assert_eq!(PERMANENT_REDIRECT, 308);
    }
}
