use crate::app::Message;
use crate::runtime::Runtime;
use crate::translations::I18nManager;
use iced::widget::PickList;
use iced::widget::row;
use iced::widget::{Text, button, column, scrollable, space};
use iced::{Center, Element, Fill};

pub fn view(i18n: &I18nManager, runtime: &Runtime) -> Element<'static, Message> {
    let items_for_display: Vec<String> = runtime
        .profile_list
        .iter()
        .map(|profile| profile.name.clone())
        .collect();

    let selected_label = runtime
        .selected_profile
        .as_ref()
        .map(|profile| profile.name.clone());

    let pick_list = PickList::new(items_for_display, selected_label, Message::ProfileChosen);

    let confirm_profile_select_button =
        button(Text::new(i18n.text("select-button"))).on_press(Message::ConfirmProfileSelection);

    let new_profile_button =
        button(Text::new(i18n.text("new-profile-button"))).on_press(Message::NewProfileSetup);

    let cancel_button =
        button(Text::new(i18n.text("cancel-button"))).on_press(Message::CancelButton);

    let button_row = row![
        confirm_profile_select_button,
        new_profile_button,
        cancel_button
    ]
    .spacing(10)
    .align_y(Center);

    let content = column![
        space().height(10),
        Text::new(i18n.text("select-profile")),
        pick_list,
        button_row
    ]
    .width(Fill)
    .align_x(Center)
    .spacing(10);

    scrollable(content).into()
}
