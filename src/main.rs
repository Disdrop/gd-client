use leptos::logging;
use leptos::*;

mod component {
  pub mod window_container;
  pub use window_container::WindowContainer;
}

mod app;
pub use app::App;
use stylers::style_sheet;

fn main() {
  console_error_panic_hook::set_once();
  let global_css = style_sheet!("./src/global.css");
  logging::log!("global_css: {global_css}");
  mount_to_body(|| {
    view! { class=global_css, <App/> }
  })
}
