use yew::prelude::*;

use crate::components::header::Header;
use crate::components::rustcean_list::RustceanList;
use crate::components::sidebar::Sidebar;

#[function_component(Rustaceans)]
pub fn rustaceans() -> Html {
    html! {
        <div class="container">
            <div class="row">
                <div class="col-sm-auto">
                    <Sidebar />
                </div>
                <div class="col mt-3">
                    <Header />
                    <RustceanList />
                </div>
            </div>
        </div>
    }
}
