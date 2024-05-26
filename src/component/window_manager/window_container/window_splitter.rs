use leptos::*;
use stylers::style;

use crate::component::window_manager::model;

#[component]
pub fn WindowSplitter(window_splitter: model::WindowSplitter) -> impl IntoView {
  let styler_class = style! { "WindowSplitter",
    .window-splitter {
      width: 100%;
      height: 100%;
      padding: 4px;
      border-style: solid;
      border-width: 2px;
      display: flex;
      flex-direction: column;
      gap: 4px;
    }
    .window-splitter-row {
      border-color: blue;
      flex-direction: row;
    }
    .window-splitter-column {
      border-color: green;
      flex-direction: column;
    }
  };

  view! { class=styler_class,
    <div class="window-splitter">
      <div>
        <For
          // a function that returns the items we're iterating over; a signal is fine
          each=move || window_splitter.children.clone()
          // a unique key for each item
          key=|counter| counter.id
          // renders each item to a view
          children=move |child: model::WindowSplitter| {
              view! { <WindowSplitter window_splitter=child/> }
          }
        />

      </div>
    </div>
  }
}
