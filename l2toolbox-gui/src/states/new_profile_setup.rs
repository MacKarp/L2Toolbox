use crate::app::Message;
use crate::runtime::Runtime;
use crate::translations::I18nManager;
use iced::Center;
use iced::Element;
use iced::widget::{Text, button, checkbox, column, row, text_input};

pub fn view(i18n: &I18nManager, runtime: &Runtime) -> Element<'static, Message> {
    let profile_name = runtime
        .profile_draft
        .as_ref()
        .map(|draft| draft.profile_name.clone())
        .unwrap_or_default();

    let lineage2_directory_path = runtime
        .profile_draft
        .as_ref()
        .map(|draft| draft.lineage2_path.clone())
        .unwrap_or_default();

    let lineage2_system_path = runtime
        .profile_draft
        .as_ref()
        .and_then(|draft| draft.system_path.clone())
        .unwrap_or_default();

    let custom_subdirectory = runtime
        .profile_draft
        .as_ref()
        .map(|draft| draft.custom_subdir)
        .unwrap_or(false);

    let profile_name_row: Element<Message> =
        row![text_input("Profile name", &profile_name).on_input(Message::ProfileNameChanged),]
            .into();

    let main_directory_row: Element<Message> = row![
        text_input(
            &i18n.text("select-lineage2-main-directory"),
            &lineage2_directory_path
        ),
        button(Text::new(i18n.text("pick-button"))).on_press(Message::PickLineage2)
    ]
    .into();
    let main_directory_column = column![
        Text::new("Select Lineage 2 main directory"),
        main_directory_row
    ];

    let custom_dirs_row: Element<Message> = row![
        checkbox("Use custom subdirectories", custom_subdirectory)
            .on_toggle(Message::CustomSubdirectoryToggled),
    ]
    .into();

    let custom_dirs_choice_row: Element<Message> = if custom_subdirectory {
        row![
            text_input(
                &i18n.text("select-lineage2-system-directory"),
                &lineage2_system_path
            ),
            button("Pick").on_press(Message::PickLineage2System)
        ]
        .into()
    } else {
        row![].into()
    };

    let ok_button =
        button(Text::new(i18n.text("ok-button"))).on_press(Message::NewProfileSetupComplete);
    let cancel_button =
        button(Text::new(i18n.text("cancel-button"))).on_press(Message::CancelButton);
    let button_row = row![ok_button, cancel_button].spacing(10).align_y(Center);

    column![
        Text::new(i18n.text("select-profile")),
        profile_name_row,
        main_directory_column,
        custom_dirs_row,
        custom_dirs_choice_row,
        button_row
    ]
    .spacing(15)
    .padding(20)
    .into()
}
