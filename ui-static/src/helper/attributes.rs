use another_html_builder::attribute::AttributeValue;

pub struct Concat<A, B>(pub A, pub B);

impl<'a, 'b> From<(&'a str, &'b str)> for Concat<&'a str, &'b str> {
    fn from((first, second): (&'a str, &'b str)) -> Self {
        Self(first, second)
    }
}

impl<A: std::fmt::Display, B: std::fmt::Display> AttributeValue for Concat<A, B> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}
