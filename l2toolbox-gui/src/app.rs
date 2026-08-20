use crate::config::ApplicationConfig;
use crate::profiles;
use crate::profiles::config::ProfileConfig;
use crate::runtime::Runtime;
use crate::states::{main_app, new_profile_setup, profile_selection, translation_selection};
use crate::translations::I18nManager;
use iced::Task;

use iced::Element;
use unic_langid::LanguageIdentifier;

#[derive(Debug)]
pub struct App {
    state: AppState,
    message: Message,
    config: ApplicationConfig,
    i18n: I18nManager,
    runtime: Runtime,
}
#[allow(dead_code)]
#[derive(Debug)]
enum AppState {
    TranslationSelection,
    NewProfileSetup,
    MainApp,
    ProfileSelection,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    LanguageSelected(LanguageIdentifier),
    ConfigSaveButtonPressed,
    NewProfileSetup,
    ProfileChosen(String),
    ConfirmProfileSelection,
    CancelButton,

    ProfileNameChanged(String),
    PickLineage2,
    Lineage2Selected(Option<std::path::PathBuf>),
    CustomSubdirectoryToggled(bool),
    PickLineage2System,
    Lineage2SystemSelected(Option<std::path::PathBuf>),
    NewProfileSetupComplete,
}

impl App {
    pub fn initialize() -> (App, iced::Task<Message>) {
        println!("ℹ️ Loading application config file!");
        let config = ApplicationConfig::load_or_create()
            .expect("❌ Failed to load application configuration!");

        println!("ℹ️ Initializing runtime...");
        let mut runtime = Runtime::initialize(&config).expect("❌ Failed to initialize runtime");

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

        let (state, profile_config) = match &runtime.current_profile {
            Some(profile) => {
                println!(
                    "ℹ️ Loading profile configuration: '{}' ({})",
                    profile.name, profile.id
                );

                match profiles::config::load(profile) {
                    Ok(config) => {
                        println!("✅ Profile configuration loaded successfully.");

                        (AppState::MainApp, Some(config))
                    }
                    Err(err) => {
                        eprintln!(
                            "❌ Failed to load profile '{}' ({}): {err}",
                            profile.name, profile.id
                        );

                        runtime.profile_draft = Some(ProfileConfig::default());

                        (AppState::NewProfileSetup, None)
                    }
                }
            }

            None => {
                println!("ℹ️ No profile selected. Starting new profile setup.");

                runtime.profile_draft = Some(ProfileConfig::default());

                (AppState::NewProfileSetup, None)
            }
        };

        if let Some(config) = &profile_config {
            runtime.loaded_profile = Some(config.clone());
        }

        let app = App {
            state,
            message: Message::None,
            config,
            i18n,
            runtime,
        };
        println!("✅ Initialize complete! Starting application...");
        (app, iced::Task::none())
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::None => Task::none(),
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
                Task::none()
            }
            Message::ConfigSaveButtonPressed => {
                if let Err(e) = ApplicationConfig::save_config(&self.config) {
                    eprintln!("❌ Failed to save config: {e}");
                } else {
                    println!("✅ Config saved successfully!");
                    self.message = Message::None;
                }
                Task::none()
            }
            Message::NewProfileSetup => {
                println!("ℹ️ Starting new profile setup");

                self.runtime.profile_draft = Some(ProfileConfig::default());

                self.state = AppState::NewProfileSetup;

                Task::none()
            }

            Message::ProfileChosen(profile_name) => {
                self.runtime.selected_profile = self
                    .runtime
                    .profile_list
                    .iter()
                    .find(|profile| profile.name == profile_name)
                    .cloned();

                Task::none()
            }
            Message::ConfirmProfileSelection => {
                let Some(profile) = self.runtime.selected_profile.clone() else {
                    eprintln!("❌ No profile selected!");
                    return Task::none();
                };

                self.runtime.current_profile = Some(profile.clone());

                self.config.last_profile_id = Some(profile.id.clone());

                if let Err(e) = ApplicationConfig::save_config(&self.config) {
                    eprintln!("❌ Failed to save config: {e}");
                } else {
                    println!("✅ Config saved successfully!");
                    println!("✅ Running Main window!");

                    match profiles::config::load(&profile) {
                        Ok(config) => {
                            self.runtime.loaded_profile = Some(config.clone());
                            self.state = AppState::MainApp;
                        }
                        Err(err) => {
                            eprintln!("❌ Failed to load profile: {err}");
                        }
                    }
                }

                Task::none()
            }
            Message::CancelButton => {
                println!("Cancel Button pressed");
                Task::none()
            }
            Message::ProfileNameChanged(name) => {
                if let Some(draft) = &mut self.runtime.profile_draft {
                    draft.profile_name = name;
                }

                Task::none()
            }
            Message::PickLineage2 => {
                return Task::perform(
                    pick_lineage2_folder(),
                    Message::Lineage2Selected, // Map the result to this message
                );
            }
            Message::Lineage2Selected(opt_path) => {
                if let Some(path) = opt_path {
                    println!("Directory selected: {:?}", path);

                    if let Some(draft) = &mut self.runtime.profile_draft {
                        draft.lineage2_path = path.to_string_lossy().to_string();
                    }
                }

                Task::none()
            }
            Message::PickLineage2System => {
                Task::perform(pick_lineage2_folder(), Message::Lineage2SystemSelected)
            }
            Message::Lineage2SystemSelected(opt_path) => {
                if let Some(path) = opt_path {
                    println!("Directory selected: {:?}", path);

                    if let Some(draft) = &mut self.runtime.profile_draft {
                        draft.system_path = Some(path.to_string_lossy().to_string());
                    }
                }

                Task::none()
            }
            Message::CustomSubdirectoryToggled(value) => {
                if let Some(draft) = &mut self.runtime.profile_draft {
                    draft.custom_subdir = value;
                }

                Task::none()
            }
            Message::NewProfileSetupComplete => {
                println!("✅ New Profile creation requested");

                let Some(draft) = self.runtime.profile_draft.clone() else {
                    eprintln!("❌ No profile draft exists!");
                    return Task::none();
                };

                if draft.profile_name.trim().is_empty() {
                    eprintln!("❌ Profile name cannot be empty!");
                    return Task::none();
                }

                if draft.lineage2_path.trim().is_empty() {
                    eprintln!("❌ Lineage 2 path cannot be empty!");
                    return Task::none();
                }

                let profiles_dir = match self.runtime.application_config_dir.parent() {
                    Some(parent) => parent.join("profiles"),
                    None => {
                        eprintln!("❌ Failed to determine profiles directory!");
                        return Task::none();
                    }
                };

                let id = uuid::Uuid::new_v4().to_string();
                let profile_dir = profiles_dir.join(&id);

                if let Err(e) = draft.save(&profile_dir) {
                    eprintln!("❌ Failed to save profile: {e}");
                    return Task::none();
                }

                let profile = crate::profiles::types::ProfileEntry::new(
                    id,
                    draft.profile_name.clone(),
                    profile_dir,
                );

                println!("✅ Profile saved: {}", profile.name);

                self.runtime.profile_list.push(profile.clone());
                self.runtime.current_profile = Some(profile.clone());
                self.runtime.selected_profile = Some(profile.clone());
                self.runtime.loaded_profile = Some(draft.clone());

                self.config.last_profile_id = Some(profile.id.clone());

                if let Err(e) = ApplicationConfig::save_config(&self.config) {
                    eprintln!("⚠️ Profile created, but failed to save last profile: {e}");
                }

                self.runtime.profile_draft = None;
                self.state = AppState::MainApp;

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.state {
            AppState::TranslationSelection => translation_selection::view(&self.config, &self.i18n),
            AppState::NewProfileSetup => new_profile_setup::view(&self.i18n, &self.runtime),
            AppState::MainApp => main_app::view(),
            AppState::ProfileSelection => profile_selection::view(&self.i18n, &self.runtime),
        }
    }
}

async fn pick_lineage2_folder() -> Option<std::path::PathBuf> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Select Lineage 2 Directory")
        .pick_folder()
        .await;

    handle.map(|h| h.path().to_path_buf())
}
