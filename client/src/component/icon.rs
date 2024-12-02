use std::fmt::Write;

#[derive(Debug, Clone, Copy)]
pub enum IconKind {
    Battery,
    Dashboard,
    Sun,
    TemperatureHot,
    Time,
    Water,
}

impl IconKind {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Battery => "ri-battery-2-line",
            Self::Dashboard => "ri-dashboard-2-line",
            Self::Sun => "ri-sun-line",
            Self::TemperatureHot => "ri-temp-hot-line",
            Self::Time => "ri-time-line",
            Self::Water => "ri-drop-line",
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum IconSize {
    #[default]
    Md,
}

impl IconSize {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Md => "ri-1x",
        }
    }
}

struct IconClassName {
    kind: IconKind,
    size: IconSize,
}

impl another_html_builder::AttributeValue for IconClassName {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("icon ")?;
        f.write_str(self.kind.as_str())?;
        f.write_char(' ')?;
        f.write_str(self.size.as_str())?;
        Ok(())
    }
}

pub struct Icon {
    kind: IconKind,
    size: IconSize,
}

impl Icon {
    pub fn new(kind: IconKind) -> Self {
        Self {
            kind,
            size: IconSize::Md,
        }
    }
}

impl crate::component::prelude::Component for Icon {
    fn render<'v, W: std::fmt::Write>(
        &self,
        buf: another_html_builder::Buffer<W, another_html_builder::Body<'v>>,
    ) -> another_html_builder::Buffer<W, another_html_builder::Body<'v>> {
        buf.node("i")
            .attr((
                "class",
                IconClassName {
                    kind: self.kind,
                    size: self.size,
                },
            ))
            .content(|buf| buf)
    }
}
