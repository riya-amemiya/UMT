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
