#[derive(Copy, Clone, Debug, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TimeRange {
    #[default]
    Hour,
    Day,
    Week,
    Month,
}

impl TimeRange {
    pub const fn duration(&self) -> u64 {
        match self {
            Self::Hour => 60 * 60,
            Self::Day => 60 * 60 * 24,
            Self::Week => 60 * 60 * 24 * 7,
            Self::Month => 60 * 60 * 24 * 7 * 4,
        }
    }

    pub fn as_secs(&self) -> (u64, u64) {
        let ends = chezmoi_entity::now();
        (ends - self.duration(), ends)
    }

    pub const fn partitions(&self) -> usize {
        match self {
            Self::Hour => 30,
            Self::Day => 48,
            Self::Week => 56,
            Self::Month => 31 * 2,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, serde::Deserialize)]
pub struct SharedParams {
    #[serde(default)]
    pub timerange: TimeRange,
}
