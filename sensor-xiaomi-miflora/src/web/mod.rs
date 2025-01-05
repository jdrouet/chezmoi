use chezmoi_entity::metric::MetricHeader;
use chezmoi_web_prelude::helper::range::Range;

pub mod board;
pub mod card;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Definition {
    #[serde(default)]
    pub name: Option<String>,
    pub address: String,
    #[serde(default)]
    pub temperature: Range<f64>,
    #[serde(default)]
    pub brightness: Range<f64>,
    #[serde(default)]
    pub conductivity: Range<f64>,
    #[serde(default)]
    pub moisture: Range<f64>,
    #[serde(default)]
    pub battery: Range<f64>,
}

#[derive(Clone, Debug)]
pub struct Headers {
    pub temperature: MetricHeader<'static>,
    pub brightness: MetricHeader<'static>,
    pub conductivity: MetricHeader<'static>,
    pub moisture: MetricHeader<'static>,
    pub battery: MetricHeader<'static>,
}

impl Headers {
    pub fn new(def: &Definition) -> Self {
        Self {
            temperature: MetricHeader::new(crate::TEMPERATURE)
                .with_tag("address", def.address.clone()),
            brightness: MetricHeader::new(crate::BRIGHTNESS)
                .with_tag("address", def.address.clone()),
            conductivity: MetricHeader::new(crate::CONDUCTIVITY)
                .with_tag("address", def.address.clone()),
            moisture: MetricHeader::new(crate::MOISTURE).with_tag("address", def.address.clone()),
            battery: MetricHeader::new(crate::BATTERY).with_tag("address", def.address.clone()),
        }
    }

    pub fn list(&self) -> impl Iterator<Item = &MetricHeader<'static>> {
        [
            &self.temperature,
            &self.brightness,
            &self.conductivity,
            &self.moisture,
            &self.battery,
        ]
        .into_iter()
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(from = "Definition")]
pub struct Config {
    pub definition: Definition,
    pub headers: Headers,
}

impl From<Definition> for Config {
    fn from(value: Definition) -> Self {
        let headers = Headers::new(&value);
        Self {
            definition: value,
            headers,
        }
    }
}
