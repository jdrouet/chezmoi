use chezmoi_client::component::any_card::AnyCard;
use chezmoi_client::component::miflora_card::{LastValues, MifloraCard};
use chezmoi_client::view::home::{Section, View};

mod helper;

#[test]
fn with_miflora_cards() {
    helper::write(
        "with-miflora-cards.html",
        View::new(helper::STYLE_PATH)
            .with_section(
                Section::new("With last values")
                    .with_card(AnyCard::Miflora(MifloraCard::new(
                        "00:00:00:00:00",
                        Some("Orchidee"),
                        Some(LastValues {
                            timestamp: 0,
                            temperature: 12.34,
                            brightness: 12.34,
                            moisture: 12.34,
                            conductivity: 12.34,
                            battery: 85.0,
                        }),
                    )))
                    .with_card(AnyCard::Miflora(MifloraCard::new(
                        "00:00:00:00:00",
                        None::<&'static str>,
                        Some(LastValues {
                            timestamp: 0,
                            temperature: 12.34,
                            brightness: 12.34,
                            moisture: 12.34,
                            conductivity: 12.34,
                            battery: 85.0,
                        }),
                    ))),
            )
            .with_section(
                Section::new("Without last values")
                    .with_card(AnyCard::Miflora(MifloraCard::new(
                        "00:00:00:00:00",
                        Some("With name"),
                        None,
                    )))
                    .with_card(AnyCard::Miflora(MifloraCard::new(
                        "00:00:00:00:00",
                        None::<&'static str>,
                        None,
                    ))),
            ),
    );
}
