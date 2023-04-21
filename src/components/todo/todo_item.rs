use yew::{function_component, html, Html, Properties, Callback, MouseEvent};

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
  pub title: String,
  pub id: usize,
  pub completed: bool,  
  pub on_done: Callback<String>
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {

  let onclick = move |id: usize| {
    let on_done = props.on_done.clone();
    Callback::from(move |e: MouseEvent| {
      e.prevent_default();
      on_done.emit(id.to_string());
    })
  };
  html! {
    <li class="p-2">
      {&props.title}
      <button type="submit" onclick={onclick(props.id)} class="btn btn-sm btn-danger mr-2">{"完了"}</button>
    </li>
  }
}