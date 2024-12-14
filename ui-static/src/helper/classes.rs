use std::fmt::Write;

use another_html_builder::attribute::AttributeValue;

pub struct Cn<A, B>(pub A, pub B);

impl<A: AttributeValue, B: AttributeValue> AttributeValue for Cn<A, B> {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.render(f)?;
        f.write_char(' ')?;
        self.1.render(f)
    }
}
