use crate::app::Message;
use iced::{Element, widget::Column, widget::Text};

pub fn view() -> Element<'static, Message> {
    Column::new()
        .push(Text::new("Profile Selection Screen"))
        .into()
}
