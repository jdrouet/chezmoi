use std::sync::LazyLock;

pub static TEMPERATURE: LazyLock<human_number::Formatter<'static>> = LazyLock::new(|| {
    human_number::Formatter::si()
        .with_decimals(1)
        .with_unit("°C")
});

pub static PERCENTAGE: LazyLock<human_number::Formatter<'static>> = LazyLock::new(|| {
    human_number::Formatter::si()
        .with_decimals(1)
        .with_unit("%")
});
