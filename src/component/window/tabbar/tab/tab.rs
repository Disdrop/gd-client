use leptos::*;

use stylers::style;

#[component]
pub fn Tab() -> impl IntoView {
  let styler_class = style! {"Tab",
    .tab {
      width: 132px;
      background: white;
      border-style: solid;
      border-width: 1px;
      border-radius: 6px;
      border-color: #ccc;
    }
  };
  view! { class=styler_class,
    <div class="tab">
      <span class="tab-title">{}</span>
      <span class="tab-close">x</span>
    </div>
  }
}
