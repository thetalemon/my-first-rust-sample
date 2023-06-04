use yew::{function_component, html, Html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
      <header class="p-2 color-bg-accent">
        <div>
          <a  href="#">{"Yew TODO App"}</a>
        </div>
      </header>
    }
}