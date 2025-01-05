use chezmoi_entity::metric::MetricHeader;
use chezmoi_web_prelude::helper::range::Range;

pub mod card;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Definition {
    #[serde(default)]
    pub name: Option<String>,
    pub address: String,
    #[serde(default)]
    pub temperature: Range<f64>,
    #[serde(default)]
    pub humidity: Range<f64>,
    #[serde(default)]
    pub battery: Range<f64>,
}

#[derive(Clone, Debug)]
pub struct Headers {
    pub temperature: MetricHeader<'static>,
    pub humidity: MetricHeader<'static>,
    pub battery: MetricHeader<'static>,
}

impl Headers {
    pub fn new(def: &Definition) -> Self {
        Self {
            temperature: MetricHeader::new(crate::TEMPERATURE)
                .with_tag("address", def.address.clone()),
            humidity: MetricHeader::new(crate::HUMIDITY).with_tag("address", def.address.clone()),
            battery: MetricHeader::new(crate::BATTERY).with_tag("address", def.address.clone()),
        }
    }

    pub fn list(&self) -> impl Iterator<Item = &MetricHeader<'static>> {
        [&self.temperature, &self.humidity, &self.battery].into_iter()
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
