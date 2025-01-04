use another_html_builder::attribute::AttributeValue;

pub struct Concat<A, B>(pub A, pub B);

impl<A: std::fmt::Display, B: std::fmt::Display> From<(A, B)> for Concat<A, B> {
    fn from((first, second): (A, B)) -> Self {
        Self(first, second)
    }
}

impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Concat<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl<A: std::fmt::Display, B: std::fmt::Display> AttributeValue for Concat<A, B> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}
