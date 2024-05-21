use leptos::logging;
use leptos::*;

mod component {
  mod window {
    pub mod window;
    pub use window::Window;
  }
  pub mod window_container;
  pub use window_container::WindowContainer;
  
}

mod app;
pub use app::App;

use stylers::style_sheet;

fn main() {
  console_error_panic_hook::set_once();
  mount_to_body(|| {
    view! { class=global_css, <App/> }
  })
}
