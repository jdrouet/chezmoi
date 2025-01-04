#[cfg(feature = "agent")]
pub mod agent;

#[cfg(feature = "web")]
pub mod web;

pub const BATTERY: &str = "miflora.battery";
pub const TEMPERATURE: &str = "miflora.temperature";
pub const BRIGHTNESS: &str = "miflora.brightness";
pub const CONDUCTIVITY: &str = "miflora.conductivity";
pub const MOISTURE: &str = "miflora.moisture";
