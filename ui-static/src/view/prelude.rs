pub trait View {
    fn render<P>(&self, props: &P) -> String;
}
