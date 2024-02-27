mod api;
mod components;
mod pages;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <pages::login::Login />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
