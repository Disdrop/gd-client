use leptos::*;

use stylers::*;

use crate::component::WindowContainer;

#[component]
pub fn App() -> impl IntoView {
  let global_css = style_sheet!("./src/global.css");
  let styler_class = style! { "App",
    #toolbar {
      background-color: #DCDCE1;
      border-style: solid;
      border-width: 0 0 1px;
      border-color: #DCDCE1;
    }
  };
  view! { class=styler_class,
    <div id="toolbar"></div>
    <WindowContainer/>
  }
}
