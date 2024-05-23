use leptos::*;

use stylers::style;

use crate::component::window_manager::model;

pub mod window;
pub mod window_splitter;
use window::Window;
use window_splitter::WindowSplitter;

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

  let windows = vec![
    vec![
      model::Window {
        id: 1,
        tabs: tabs.clone(),
      },
      model::Window {
        id: 2,
        tabs: tabs.clone(),
      },
    ],
    vec![model::Window {
      id: 3,
      tabs: tabs.clone(),
    }],
  ];

  let (editors, _) = create_signal::<Vec<model::Tab>>(tabs);

  let (windows_order, set_window_order) = create_signal::<Vec<Vec<model::Window>>>(windows);

  provide_context(set_window_order);
  // let (windows, _) = create_signal::<Vec<Editor>>(editors);

  view! { class=styler_class,
    <div class="window-container">
      <For each=move || windows_order.get() key=|n| n.clone() let:window_column>
        <div class="window-row">
          <For each=move || window_column.clone() key=|n| n.clone() let:window>
            <div class="window-column">
              <Window editor=window.tabs.clone()/>
            </div>
          </For>
        </div>
      </For>

    </div>
  }
}

// />

// <For
//   // a function that returns the items we're iterating over; a signal is fine
//   each={editors}
//   // a unique key for each item
//   key=|counter| counter.id
//   // renders each item to a view
//   children=move |counter: Editor| {
//     view! {
//       <Window editor={counter}/>
//     }
//   }
// />
