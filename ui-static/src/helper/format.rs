use std::sync::LazyLock;

pub static TEMPERATURE: LazyLock<human_number::Formatter<'static>> = LazyLock::new(|| {
    human_number::Formatter::si()
        .with_decimals(1)
        .with_unit("°C")
});

pub static PERCENTAGE: LazyLock<human_number::Formatter<'static>> = LazyLock::new(|| {
    human_number::Formatter::new(
        human_number::Scales::empty(),
        human_number::Options::default().with_unit("%"),
    )
});

pub static BRIGHTNESS: LazyLock<human_number::Formatter<'static>> = LazyLock::new(|| {
    human_number::Formatter::si()
        .with_unit("lx")
        .with_decimals(1)
});

pub static CONDUCTIVITY: LazyLock<human_number::Formatter<'static>> = LazyLock::new(|| {
    human_number::Formatter::si()
        .with_unit("S/m")
        .with_decimals(1)
});
