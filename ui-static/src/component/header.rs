// <header>
//     <div class="container flex-row space-between padx-md">
//         <a class="text-bold text-lg text-nodeco" href="/">{"Chezmoi"}</a>
//     </div>
// </header>

use another_html_builder::prelude::WriterExt;
use another_html_builder::{Body, Buffer};

pub fn render<'a, W>(buf: Buffer<W, Body<'a>>) -> Buffer<W, Body<'a>>
where
    W: WriterExt,
{
    buf.node("header").content(|buf| {
        buf.node("div")
            .attr(("class", "container flex-row space-between padx-md"))
            .content(|buf| {
                buf.node("a")
                    .attr(("class", "text-bold text-lg text-nodeco"))
                    .attr(("href", "/"))
                    .content(|buf| buf.text("Chezmoi"))
            })
    })
}
