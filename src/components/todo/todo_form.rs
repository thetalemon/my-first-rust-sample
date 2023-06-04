use yew::{Callback, InputEvent, MouseEvent, Properties, Html, function_component, html, use_state, TargetCast};
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct TodoFormProps {
  pub on_add: Callback<String>
}

#[function_component(TodoForm)]
pub fn todo_form(props: &TodoFormProps) -> Html {
  let input = use_state(|| "".to_string());

  let oninput = {
    let input = input.clone();
    Callback::from(move |e: InputEvent| {
      let value = e.target_dyn_into::<HtmlInputElement>();

      match value {
        Some(value) => {
          input.set(value.value());
        }
        None => {
          input.set("".to_string());
        }
      }
    })
  };

  let onclick = {
    let on_add = props.on_add.clone();
    let input = input.clone();
    Callback::from(move |e: MouseEvent| {
      e.prevent_default();
      if input.is_empty() {
        return
      }
      input.set("".to_string());
      on_add.emit((*input).clone());
    })
  };

  html! {
    <form class="mb-5">
      <div class="mb-3">
        <label for="title" class="form-label">{"タイトル"}</label>
        <input type="text" value={(*input).clone()} {oninput} class="form-control" id="title" />
      </div>
      <div class="mb-3">
        {&*input}
      </div>
      <button type="submit" {onclick} class="btn btn-primary">{"追加"}</button>
    </form>
  }
}