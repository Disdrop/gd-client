use leptos::*;

use stylers::style;

mod window_container;
pub use window_container::WindowContainer;

pub mod model;

pub use window_container::window::Window;
pub use window_container::window_splitter::WindowSplitter;

#[component]
pub fn WindowManager() -> impl IntoView {
  let styler_class = style! { "WindowManager",
    .window-manager {
      padding: 4px;
      border-style: solid;
      border-width: 2px;
      border-color: red;
      display: flex;
      gap: 4px;
    }
  };

  view! { class=styler_class,
    <div class="window-manager">
      // <WindowContainer/>
      <WindowContainer/>
    // <WindowContainer/>
    </div>
  }
}
