use leptos::*;
use stylers::style;

use crate::component::window_manager::model;
use crate::component::window_manager::window_container::Window;

#[component]
pub fn WindowSplitter() -> impl IntoView {
  let styler_class = style! { "WindowSplitter",
    .window-splitter {
      width: 100%;
      height: 100%;
      padding: 4px;
      border-style: solid;
      border-width: 2px;
      display: flex;
      flex-direction: column;
      gap: 4px;
    }
    .window-splitter-row {
      border-color: blue;
      flex-direction: row;
    }
    .window-splitter-column {
      border-color: green;
      flex-direction: column;
    }
  };
  view! { class=styler_class, <div class="window-splitter"></div> }
}
