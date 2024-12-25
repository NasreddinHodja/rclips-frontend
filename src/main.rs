mod app;

use app::App;

mod components;
mod utils;

fn main() {
    yew::Renderer::<App>::new().render();
}
