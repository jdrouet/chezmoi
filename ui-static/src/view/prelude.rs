use crate::context::Context;

pub trait View {
    fn render(&self, ctx: &Context) -> String;
}
