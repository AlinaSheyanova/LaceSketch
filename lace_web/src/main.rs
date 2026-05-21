use lace_gui::LaceEditor;

pub fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init().expect("Initialize logger");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    iced::application(LaceEditor::default, LaceEditor::update, LaceEditor::view)
        .title(LaceEditor::title)
        .centered()
        .run()
}
