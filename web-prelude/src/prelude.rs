use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

pub trait Buildable<Props> {
    fn build(props: &Props) -> Self;
}

pub trait Component: Sized {
    fn render<'a, W: WriterExt>(&self, buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>>;
}

pub trait RenderComponent {
    fn render_component<C: Component>(self, component: C) -> Self;
}

impl<'a, W: WriterExt> RenderComponent for Buffer<W, Body<'a>> {
    fn render_component<C: Component>(self, component: C) -> Self {
        component.render(self)
    }
}
