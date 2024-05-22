#[allow(unused)]
use leptos::logging;
use leptos::*;

mod component {
  mod window {
    pub mod window;
    pub use window::Window;
    mod tabbar {
      pub mod tabbar;
      pub use tabbar::Tabbar;
      mod tab {
        pub mod tab;
        pub use tab::Tab;
      }
    }
  }
  pub mod window_container;
  pub use window_container::WindowContainer;
}

mod app;
pub use app::App;

fn main() {
  console_error_panic_hook::set_once();
  mount_to_body(|| {
    view! { <App/> }
  })
}
