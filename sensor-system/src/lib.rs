#[cfg(feature = "agent")]
pub mod agent;

pub const MEMORY_TOTAL: &str = "system.memory-total";
pub const MEMORY_USED: &str = "system.memory-used";
pub const MEMORY_RATIO: &str = "system.memory-ratio";
pub const SWAP_TOTAL: &str = "system.swap-total";
pub const SWAP_USED: &str = "system.swap-used";
pub const SWAP_RATIO: &str = "system.swap-ratio";
