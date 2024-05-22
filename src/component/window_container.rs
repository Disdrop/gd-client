use leptos::*;
use stylers::style;

use crate::component::window::Window;

#[component]
pub fn WindowContainer() -> impl IntoView {
  let (windows, set_windows) = create_signal(vec![]);
  if windows.with(|windows| windows.is_empty()) {
    set_windows.update(|windows| windows.push(Window));
  }
  let styler_class = style! { "WindowContaine",
    #window-container {
      padding: 2px;
      display: grid;
      gap: 10px;
    }
  };
  view! { class=styler_class, <div id="window-container">{windows}</div> }
}
