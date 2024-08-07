mod app;
mod components;
mod pages;
mod router;
use app::App;
use log::Level;


fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::Renderer::<App>::new().render();
}
