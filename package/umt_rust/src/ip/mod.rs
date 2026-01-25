//! IP address manipulation utilities
//!
//! This module provides functions for working with IPv4 addresses,
//! including conversion, validation, and network calculations.

mod cidr_to_long;
mod cidr_to_subnet_mask;
mod get_ip_class;
mod get_network_address;
mod ip_to_binary_string;
mod ip_to_long;
mod is_in_range;
mod is_private_ip;
mod long_to_ip;
mod subnet_mask_to_cidr;

pub use cidr_to_long::cidr_to_long;
pub use cidr_to_subnet_mask::cidr_to_subnet_mask;
pub use get_ip_class::get_ip_class;
pub use get_network_address::get_network_address;
pub use ip_to_binary_string::ip_to_binary_string;
pub use ip_to_long::ip_to_long;
pub use is_in_range::is_in_range;
pub use is_private_ip::is_private_ip;
pub use long_to_ip::long_to_ip;
pub use subnet_mask_to_cidr::subnet_mask_to_cidr;
