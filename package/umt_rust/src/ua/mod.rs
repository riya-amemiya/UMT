//! User Agent parsing module.
//!
//! This module provides functions to parse and extract information from User-Agent strings,
//! including browser, device, and operating system detection.

pub mod extract_browser_from_user_agent;
pub use extract_browser_from_user_agent::{umt_extract_browser_from_user_agent, Browser};

pub mod extract_device_from_user_agent;
pub use extract_device_from_user_agent::{umt_extract_device_from_user_agent, Device};

pub mod extract_os_from_user_agent;
pub use extract_os_from_user_agent::{umt_extract_os_from_user_agent, Os};

pub mod parse_user_agent;
pub use parse_user_agent::{umt_parse_user_agent, UserAgentInfo};
