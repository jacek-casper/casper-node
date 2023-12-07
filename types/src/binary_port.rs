//! The binary port.
pub mod binary_request;
pub mod binary_response;
pub mod binary_response_and_request;
pub mod binary_response_header;
pub mod db_id;
pub mod error_code;
pub mod get;
pub mod get_all_values;
pub mod global_state;
pub mod non_persistent_data;
pub mod payload_type;
pub mod speculative_execution;
pub mod type_wrappers;

pub use error_code::ErrorCode;
pub use payload_type::PayloadType;
pub use type_wrappers::Uptime;

use alloc::vec::Vec;

const PROTOCOL_VERSION: u8 = 0;

/// Stores raw bytes from the DB along with the flag indicating whether data come from legacy or current version of the DB.
#[derive(Debug)]
pub struct DbRawBytesSpec {
    is_legacy: bool,
    raw_bytes: Vec<u8>,
}

impl DbRawBytesSpec {
    /// Creates a variant indicating that raw bytes are coming from the legacy database.
    pub fn new_legacy(raw_bytes: &[u8]) -> Self {
        Self {
            is_legacy: true,
            raw_bytes: raw_bytes.to_vec(),
        }
    }

    /// Creates a variant indicating that raw bytes are coming from the current database.
    pub fn new_current(raw_bytes: &[u8]) -> Self {
        Self {
            is_legacy: false,
            raw_bytes: raw_bytes.to_vec(),
        }
    }
}
