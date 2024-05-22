use leptos::*;

use stylers::style;

use crate::component::WindowContainer;

#[component]
pub fn App() -> impl IntoView {
  let styler_class = style! { "App",
    #toolbar {
      background-color: #DCDCE1;
      border-style: solid;
      border-width: 0 0 1px;
      border-color: #999999;
    }
  };
  view! { class=styler_class,
    <div id="toolbar"></div>
    <WindowContainer/>
    //<WindowContainer/>
    //<WindowContainer/>
  }
}
