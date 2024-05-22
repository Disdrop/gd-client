use leptos::*;

use stylers::style;

use crate::component::window::tabbar::tab::Tab;

#[component]
pub fn Tabbar() -> impl IntoView {
  let (tabs, set_tabs) = create_signal(vec![]);
  if tabs.with(|tabs| tabs.is_empty()) {
    set_tabs.update(|tabs| tabs.push(Tab));
  }
  let styler_class = style! {"Tabbar",
    .tabbar {
      display: flex;
      gap: 3px;
    }
  };
  view! { class=styler_class, <div class="tabbar">{tabs}</div> } 
}
