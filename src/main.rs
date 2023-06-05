use yew::prelude::*;
mod components;
use components::todo::todo_list::TodoList;
use components::todo::todo_form::TodoForm;
use crate::components::todo::types::Todo;
use crate::components::header::Header;

#[function_component(App)]
fn app() -> Html {
  let todo_items = use_state(|| Vec::<Todo>::new());
  let next_id = use_state(|| 1);

  let on_add = {
    let todo_items = todo_items.clone();
    Callback::from(move |title: String| {
      let mut current_todo_items = (*todo_items).clone();
      current_todo_items.push(Todo {
        id: *next_id,
        title,
        completed: false,
      });
      next_id.set(*next_id + 1);
      todo_items.set(current_todo_items);
    })
  };

  let on_done = {
    let todo_items = todo_items.clone();
    Callback::from(move |id: String| {
      let mut current_todo_items = (*todo_items).clone();
      let index = current_todo_items.iter().position(|todo| todo.id.to_string() == id).unwrap();
      current_todo_items[index].completed = true;
      // リストから削除する場合はこっち↓
      // current_todo_items.remove(index);
      todo_items.set(current_todo_items);
    })
  };

  html! {
    <>
      <Header/>
      // <header class="p-2 color-bg-accent">
      //   <div>
      //     <a  href="#">{"Yew TODO App"}</a>
      //   </div>
      // </header>
      <main class="m-2 ">
        <TodoForm {on_add} />
        <TodoList todo_items={(*todo_items).clone()} {on_done} />
      </main>
    </>
  }
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}