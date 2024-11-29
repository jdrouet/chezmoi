use std::sync::LazyLock;

use human_number::Formatter;

pub(crate) static TEMPERATURE: LazyLock<Formatter<'static>> =
    LazyLock::new(|| Formatter::si().with_unit("°C"));
pub(crate) static BRIGHTNESS: LazyLock<Formatter<'static>> =
    LazyLock::new(|| Formatter::si().with_unit("lx"));
pub(crate) static CONDUCTIVITY: LazyLock<Formatter<'static>> =
    LazyLock::new(|| Formatter::si().with_unit("S/m"));
pub(crate) static PERCENTAGE: LazyLock<Formatter<'static>> =
    LazyLock::new(|| Formatter::si().with_unit("%"));
pub(crate) static BYTES: LazyLock<human_number::Formatter<'static>> = LazyLock::new(|| {
    human_number::Formatter::binary()
        .with_unit("B")
        .with_decimals(1)
});
