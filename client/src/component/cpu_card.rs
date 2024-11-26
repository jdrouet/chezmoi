use std::sync::LazyLock;

use another_html_builder::{Body, Buffer};

static PERCENTAGE_FORMATTER: LazyLock<human_number::Formatter<'static>> = LazyLock::new(|| {
    human_number::Formatter::si()
        .with_unit("%")
        .with_decimals(1)
});

#[derive(Debug)]
pub struct CpuCard {
    usage: f64,
}

impl CpuCard {
    pub fn new(usage: f64) -> Self {
        Self { usage }
    }
}

impl super::prelude::Component for CpuCard {
    fn render<'v, W: std::fmt::Write>(&self, buf: Buffer<W, Body<'v>>) -> Buffer<W, Body<'v>> {
        buf.node("div")
            .attr((
                "class",
                "card memory-usage shadow min-w-250px h-150px m-md flex-col",
            ))
            .content(|buf| {
                buf.node("div")
                    .attr((
                        "class",
                        "card-content flex-1 text-center align-content-center",
                    ))
                    .content(|buf| {
                        buf.node("p")
                            .attr(("class", "text-xl"))
                            .content(|buf| buf.raw(PERCENTAGE_FORMATTER.format(self.usage)))
                    })
                    .node("div")
                    .attr(("class", "card-footer"))
                    .content(|buf| buf.text("CPU"))
            })
    }
}
