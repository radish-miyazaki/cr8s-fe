use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <p>{"Hello world!"}</p>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
