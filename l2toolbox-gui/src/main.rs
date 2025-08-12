mod app;
mod config;
mod states;

use app::App;

fn main() -> iced::Result {
    iced::application("L2Toolbox", App::update, App::view).run_with(App::initialize)
}
