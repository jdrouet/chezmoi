pub(crate) mod fmt;

// pub(crate) struct Classnames<'a, 'b> {
//     internal: &'a str,
//     external: Option<&'b str>,
// }

// impl<'a, 'b> Classnames<'a, 'b> {
//     pub fn new(internal: &'a str, external: Option<&'b str>) -> Self {
//         Self { internal, external }
//     }
// }

// impl another_html_builder::AttributeValue for Classnames<'_, '_> {
//     fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if let Some(external) = self.external {
//             write!(f, "{} {external}", self.internal)
//         } else {
//             write!(f, "{}", self.internal)
//         }
//     }
// }
