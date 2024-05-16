use leptos::*;

use stylers::style;

#[component]
pub fn Window() -> impl IntoView {
  let styler_class = style! { "App",
    hq {
      color: red;
    }
  };
  view! {
    class=styler_class,
    <div>
    </div>
  }
}
