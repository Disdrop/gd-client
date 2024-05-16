use leptos::*;

use stylers::style;

#[component]
pub fn WindowContainer() -> impl IntoView {
  let styler_class = style! { "WindowContainer",
    #window-container {
      padding: 10px;
      display: grid;
      gap: 10px;
    }
    div.window {

    }
  };
  view! { class=styler_class,
    <div id="window-container">
      // <Window />
      <div class="window"></div>
    </div>
  }
}
