mod app;
mod states;

use app::App;

fn main() -> iced::Result {
    iced::run("", App::update, App::view)
}
