use leptos::*;

use stylers::*;

mod window_container;

use window_container::WindowContainer;

#[component]
pub fn App() -> impl IntoView {
  //let class_name = style_sheet!("./global.css");
  let styler_class = style! { "App",
    body {
      background-color: #F0F0F5;
      height: 100vh;
      display: grid;
      grid-template-rows: 40px 1fr;
    }
    #toolbar {
      background-color: #DCDCE1;
      border-style: solid;
      border-width: 0 0 1px;
      border-color: #DCDCE1;
    }
  };
  view! { class=styler_class,
    <div id="toolbar"></div>
    <WindowContainer />
  }
}
