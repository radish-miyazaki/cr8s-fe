use yew::prelude::*;

use crate::components::header::Header;
use crate::components::sidebar::Sidebar;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
        <div class="row">
            <div class="col">
                <Sidebar />
            </div>
            <div class="col mt-3">
                <Header />
                <p>{"Have a great day!"}</p>
            </div>
        </div>
        </div>
    }
}
