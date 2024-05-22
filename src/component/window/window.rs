use leptos::*;

use stylers::style;

use crate::component::window::tabbar::Tabbar;

#[component]
pub fn Window() -> impl IntoView {
  let styler_class = style! { "Window",
    .window {
      display: grid;
      grid-template-rows: 34px 1fr;
      gap: 4px;
    }
    .window-content {
      background-color: #fff;
      border-style: solid;
      border-width: 1px;
      border-radius: 6px;
      border-color: #ccc;
    }
  };
  view! { class=styler_class,
    <div class="window">
      <Tabbar/>
      <div class="window-content"></div>
    </div>
  }
}
