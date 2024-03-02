use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::header::Header;
use crate::components::rustcean_list::RustceanList;
use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;
use crate::Route;

#[function_component(Rustaceans)]
pub fn rustaceans() -> Html {
    let current_user_context =
        use_context::<CurrentUserContext>().expect("Current user context is missing.");

    let fallback = html! { <p> {"Loading..."} </p> };

    match &current_user_context.token {
        Some(token) => {
            html! {
                <div class="container">
                    <div class="row">
                        <div class="col-sm-auto">
                            <Sidebar />
                        </div>
                        <div class="col mt-3">
                            <Header />
                            <Suspense {fallback}>
                                <RustceanList token={token.clone()} />
                            </Suspense>
                        </div>
                    </div>
                </div>
            }
        }
        None => html! {
            <Redirect<Route> to={Route::Login} />
        },
    }
}
