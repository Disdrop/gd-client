use leptos::*;

use stylers::style;

#[component]
pub fn App() -> impl IntoView {
  let styler_class = style! { "App",
    hq {
      color: red;
    }
  };

  view! { class=styler_class, <h1>"Hello World !"</h1> }
}
