use iced::theme::Theme;
use iced::widget::Text;
pub struct LaceEditor;


#[derive(Debug, Clone)]
pub enum Message {
    // Define messages for user interactions here
}


impl Default for LaceEditor {
    fn default() -> Self {
        Self
    }
}

impl LaceEditor {
    pub fn title(&self) -> String {
        "LaceSketch Editor".to_string()
    }


    pub fn theme(&self) -> iced::theme::Theme {
        Theme::Dark
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::none()
    }

    pub fn update(&mut self, _message: Message)
    {
        // Handle messages and update state here
    }

    pub fn view(&self) -> iced::Element<'_,Message> {
        Text::new("Hello, LaceSketch Editor!").into()
    }

}

