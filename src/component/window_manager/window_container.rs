use leptos::*;

use stylers::style;

use crate::component::window_manager::model;

pub mod window;
pub mod window_splitter;
use window::Window;

#[component]
pub fn WindowContainer() -> impl IntoView {
  let styler_class = style! { "WindowContainer",
    .window-container {
      width: 100%;
      height: 100%;
      padding: 4px;
      border-style: solid;
      border-width: 2px;
      border-color: yellow;
      display: flex;
      flex-direction: row;
      gap: 4px;
    }
  };

  let tabs = vec![
    model::Tab {
      title: "Tab 1".to_string(),
      content: "Hello, world1!".to_string(),
    },
    model::Tab {
      title: "Tab 2".to_string(),
      content: "Hello, 2!".to_string(),
    },
    model::Tab {
      title: "Tab 3".to_string(),
      content: "Hello, world3!".to_string(),
    },
  ];

  // let window = model::WindowOrSplitter::Window(model::Window { id: 0, tabs });

  view! { class=styler_class,
    // <Window window=window/>
    <div class="window-container"></div>
  }
}
