#[allow(unused)]
use leptos::logging;
use leptos::*;

pub mod component;

mod app;
pub use app::App;

fn main() {
  console_error_panic_hook::set_once();
  mount_to_body(|| {
    view! { <App/> }
  })
}
