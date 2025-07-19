use crate::states::{main_app, new_profile_setup, profile_selection};
use iced::Element;

#[derive(Default, Debug)]
pub struct App {
    state: AppState,
    message: Message,
}

#[derive(Default, Debug)]
enum AppState {
    #[default]
    ProfileSelection,
    NewProfileSetup,
    MainApp,
}

#[derive(Debug, Clone, Default)]
pub enum Message {
    #[default]
    NoMessage,
}

impl App {
    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Element<'_, Message> {
        match &self.state {
            AppState::ProfileSelection => profile_selection::view(),
            AppState::NewProfileSetup => new_profile_setup::view(),
            AppState::MainApp => main_app::view(),
        }
    }
}
