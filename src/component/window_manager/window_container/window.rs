use crate::component::window_manager::*;
use stylers::style;

#[component]
pub fn Window(tabs: Vec<model::Tab>) -> impl IntoView {
  // let set_window_ordner = use_context::<WriteSignal<Vec<Vec<Window>>>>().expect("to have found");

  let (selected_editor, set_selected_editor) = create_signal(tabs[0].clone());

  let styler_class = style! { "Window",
    .window {
      width: 100%;
      height: 100%;
      padding: 4px;
      border-style: solid;
      border-width: 2px;
      border-color: black;
      display: flex;
      flex-direction: column;
      gap: 4px;
    }
    window-content {
      width: 100%;
      height: 100%;
      padding: 4px;
      border-style: solid;
      border-width: 2px;
      border-color: black;
    }
  };
  view! { class=styler_class,
    <div class="window_bar">
      {selected_editor.get().title} <For each=move || tabs.clone() key=|n| n.clone() let:e>

        {
            let d = e.clone();
            view! {
              <div class="window_bar_content">
                <button on:click=move |_| {
                    set_selected_editor(e.clone());
                }>{d.title}</button>
              </div>
            }
        }

      </For>
    </div>
    <div class="window_content">
      {selected_editor.get().content + selected_editor.get().title.as_str()}
    </div>
  }
}

// #[component]
// pub fn WindowBarContent(editor: Vec<Editor>) -> impl IntoView {

//   view! {
//       <Button on_click=move || {
//         log!("clicked");
//       }>
//         {editor[0].content}
//       </Button>
//   }
// }
