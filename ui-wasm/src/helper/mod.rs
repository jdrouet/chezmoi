use std::sync::LazyLock;

pub static TEMPERATURE_UNIT: LazyLock<human_number::Formatter<'static>> = LazyLock::new(|| {
    human_number::Formatter::si()
        .with_decimals(1)
        .with_unit("°C")
});

pub static PERCENTAGE_UNIT: LazyLock<human_number::Formatter<'static>> = LazyLock::new(|| {
    human_number::Formatter::si()
        .with_decimals(1)
        .with_unit("%")
});
