mod app;
mod config;
mod profiles;
mod runtime;
mod states;
mod translations;

use app::App;

fn main() -> iced::Result {
    iced::application(App::initialize, App::update, App::view)
        .title("L2Toolbox")
        .run()
}
