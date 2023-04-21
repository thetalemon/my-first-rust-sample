use yew::{function_component, Properties, html, Html, Callback};
use crate::components::todo::todo_item::TodoItem;
use crate::components::todo::types::Todo;

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
  pub todo_items: Vec<Todo>,
  pub on_done: Callback<String>
}

#[function_component(TodoList)]
pub fn todo_list(props: &TodoItemProps) -> Html {
  html! {
    <ul class="list-group">
      {props.todo_items.iter().map(|todo| html! {
        <TodoItem 
          title={todo.title.clone()} 
          completed={todo.completed} 
          id={todo.id} 
          on_done={props.on_done.clone()} 
        />
      }).collect::<Html>()}
    </ul>
  }
}