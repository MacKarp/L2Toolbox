use crate::app::Message;
use crate::config::Config;
use crate::translations::{self, I18nManager};
use fluent::FluentArgs;
use iced::widget::{Text, button, column, pick_list, scrollable, vertical_space};
use iced::{Center, Element, Fill};

pub fn view(config: &Config, i18n: &I18nManager) -> Element<'static, Message> {
    // Prepare arguments for translation
    let mut args = FluentArgs::new();
    args.set("tabCount", "5");

    // Get available languages
    let available_languages =
        translations::get_language_names().expect("❌ Failed to get available languages!");
    println!("available_languages: {available_languages:?}");

    // Find currently selected language label
    let selected_label = available_languages
        .iter()
        .find(|(id, _)| *id == config.language)
        .map(|(_, label)| label.clone());

    // Create PickList for language selection
    let pick_list = pick_list::PickList::new(
        available_languages
            .iter()
            .map(|(_, label)| label.clone())
            .collect::<Vec<_>>(),
        selected_label,
        {
            let fallback_language = config.language.clone();
            move |selected_label| {
                let selected_id = available_languages
                    .iter()
                    .find(|(_, label)| label == &selected_label)
                    .map(|(id, _)| id.clone())
                    .unwrap_or_else(|| fallback_language.clone());

                Message::LanguageSelected(selected_id)
            }
        },
    );

    // Build UI content
    let content = column![
        vertical_space().height(10),
        Text::new(i18n.text("select-language")),
        pick_list,
        vertical_space().height(10),
        Text::new(i18n.text("fallback-key")),
        vertical_space().height(10),
        Text::new(i18n.text("non-existing-key")),
        vertical_space().height(10),
        button(Text::new(i18n.text("save-button"))).on_press(Message::ConfigSaveButtonPressed)
    ]
    .width(Fill)
    .align_x(Center)
    .spacing(10);

    scrollable(content).into()
}
