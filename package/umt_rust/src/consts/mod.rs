//! Constants module.
//!
//! This module provides commonly used constants including:
//! - Time duration constants (clock)
//! - HTTP status codes (http_status)

pub mod clock;
pub mod http_status;

pub use clock::*;
pub use http_status::HttpClientErrorStatus;
pub use http_status::HttpInformationalStatus;
pub use http_status::HttpRedirectionStatus;
pub use http_status::HttpServerErrorStatus;
pub use http_status::HttpStatus;
pub use http_status::HttpSuccessStatus;

#[cfg(test)]
mod tests {
    use super::*;

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
}
