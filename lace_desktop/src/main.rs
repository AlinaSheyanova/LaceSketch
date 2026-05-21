use lace_gui::LaceEditor;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting LaceSketch Editor Desktop...");

    iced::application(LaceEditor::default, LaceEditor::update, LaceEditor::view)
        .window(iced::window::Settings {
            size: iced::Size::new(1400.0, 900.0),
            ..Default::default()
        })
        .title(LaceEditor::title)
        .theme(LaceEditor::theme)
        .subscription(LaceEditor::subscription)
        .run()
}
