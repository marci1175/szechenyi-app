mod app;
pub mod lib;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    
    log::logger();

    yew::Renderer::<App>::new().render();
}
