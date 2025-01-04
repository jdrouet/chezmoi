#[cfg(feature = "agent")]
pub mod agent;

#[cfg(feature = "web")]
pub mod web;

pub const TEMPERATURE: &str = "xiaomi-atc.temperature";
pub const HUMIDITY: &str = "xiaomi-atc.humidity";
pub const BATTERY: &str = "xiaomi-atc.battery";
