use std::sync::atomic::{AtomicU16, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Discord epoch (2015-01-01T00:00:00.000Z)
pub const EPOCH: u64 = 1420070400000;

/// A generator for Discord snowflakes.
///
/// This structure mirrors the helper from the frontend and allows generating
/// and deconstructing snowflakes in a platform agnostic way.
#[derive(Debug)]
pub struct SnowflakeGenerator {
    worker_id: u8,
    process_id: u8,
    increment: AtomicU16,
}

impl SnowflakeGenerator {
    /// Creates a new generator with the provided worker and process ids.
    pub fn new(worker_id: u8, process_id: u8) -> Self {
        Self {
            worker_id: worker_id & 0x1F,
            process_id: process_id & 0x1F,
            increment: AtomicU16::new(0),
        }
    }

    /// Generates a new snowflake as a `u64`.
    pub fn generate(&self) -> u64 {
        let time = now_millis() - EPOCH;
        let worker = (self.worker_id as u64 & 0x1F) << 17;
        let process = (self.process_id as u64 & 0x1F) << 12;
        let inc = (self.increment.fetch_add(1, Ordering::Relaxed) as u64) & 0xFFF;
        (time << 22) | worker | process | inc
    }

    /// Deconstructs an existing snowflake into its parts.
    pub fn deconstruct(id: u64) -> DeconstructedSnowflake {
        DeconstructedSnowflake {
            timestamp: (id >> 22) + EPOCH,
            worker_id: ((id & 0x3E0000) >> 17) as u8,
            process_id: ((id & 0x1F000) >> 12) as u8,
            increment: (id & 0xFFF) as u16,
        }
    }
}

/// Parts of a Discord snowflake.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeconstructedSnowflake {
    pub timestamp: u64,
    pub worker_id: u8,
    pub process_id: u8,
    pub increment: u16,
}

impl DeconstructedSnowflake {
    /// Returns the creation time of the snowflake as [`SystemTime`].
    pub fn as_system_time(&self) -> SystemTime {
        UNIX_EPOCH + std::time::Duration::from_millis(self.timestamp)
    }
}

#[cfg(target_os = "windows")]
fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

#[cfg(not(target_os = "windows"))]
fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

/// Convenience function to parse a snowflake from a decimal string.
pub fn parse_snowflake(id: &str) -> Option<u64> {
    id.parse::<u64>().ok()
}
