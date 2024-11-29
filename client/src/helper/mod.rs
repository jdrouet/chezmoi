pub(crate) mod fmt;

// use std::fmt::Write;

// use another_html_builder::AttributeValue;

// pub(crate) struct Classnames<I, E> {
//     internal: I,
//     external: Option<E>,
// }

// impl<I: AttributeValue, E: AttributeValue> Classnames<I, E> {
//     pub fn new(internal: I, external: Option<E>) -> Self {
//         Self { internal, external }
//     }
// }

// impl<I: AttributeValue, E: AttributeValue> another_html_builder::AttributeValue
//     for Classnames<I, E>
// {
//     fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.internal.render(f)?;
//         if let Some(ref external) = self.external {
//             f.write_char(' ')?;
//             external.render(f)?;
//         }
//         Ok(())
//     }
// }
