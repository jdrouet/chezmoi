use another_html_builder::AttributeValue;

#[derive(Clone, Copy, Debug)]
pub enum Size {
    Sm,
    Md,
}

impl Size {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }
}

impl AttributeValue for Size {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Dimension {
    pub width: Size,
    pub height: Size,
}

impl Dimension {
    pub fn new(width: Size, height: Size) -> Self {
        Self { width, height }
    }
}

impl AttributeValue for Dimension {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x-{} y-{}", self.width.as_str(), self.height.as_str())
    }
}
