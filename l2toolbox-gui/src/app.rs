use crate::config::Config;
use crate::states::{main_app, new_profile_setup, translation_selection};
use crate::translations::I18nManager;

use iced::Element;
use unic_langid::LanguageIdentifier;

#[derive(Debug)]
pub struct App {
    state: AppState,
    message: Message,
    config: Config,
    i18n: I18nManager,
}

#[derive(Default, Debug)]
enum AppState {
    #[default]
    TranslationSelection,
    NewProfileSetup,
    MainApp,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    LanguageSelected(LanguageIdentifier),
    ConfigSaveButtonPressed,
}

impl App {
    pub fn initialize() -> (App, iced::Task<Message>) {
        println!("ℹ️ Initialize");

        println!("ℹ️ Loading config file!");
        let config = Config::load_or_create().expect("❌ Failed to load configuration file!");

        println!("ℹ️ Load translation file!");
        let i18n = match I18nManager::new(config.language.clone()) {
            Ok(manager) => manager,
            Err(err) => {
                eprintln!("❌ Failed to load translation file: {err}");
                panic!(
                    "❌ Initialization aborted due to error while trying to load translation file."
                );
            }
        };

        let app = App {
            state: AppState::default(),
            message: Message::None,
            config,
            i18n,
        };
        println!("✅ Initialize complete! Starting application...");
        (app, iced::Task::none())
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::None => {}
            Message::LanguageSelected(lang) => {
                println!("ℹ️ Language selected: {lang}");
                self.config.language = lang;
                self.i18n = match I18nManager::new(self.config.language.clone()) {
                    Ok(manager) => manager,
                    Err(err) => {
                        eprintln!("❌ Failed to load translation file: {err}");
                        panic!(
                            "❌ Initialization aborted due to error while trying to load translation file."
                        );
                    }
                };

                self.state = AppState::TranslationSelection;
            }
            Message::ConfigSaveButtonPressed => {
                if let Err(e) = Config::save_config(&self.config) {
                    eprintln!("❌ Failed to save config: {}", e);
                } else {
                    println!("✅ Config saved successfully!");
                    self.message = Message::None;
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.state {
            AppState::TranslationSelection => translation_selection::view(&self.config, &self.i18n),
            AppState::NewProfileSetup => new_profile_setup::view(),
            AppState::MainApp => main_app::view(),
        }
    }
}
