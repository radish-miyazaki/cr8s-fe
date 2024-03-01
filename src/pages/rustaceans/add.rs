use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::rustaceans::api_rustaceans;
use crate::components::header::Header;
use crate::components::rustacean_form::RustaceanForm;
use crate::components::sidebar::Sidebar;
use crate::contexts::CurrentUserContext;
use crate::Route;

#[function_component(RustaceansAdd)]
pub fn rustaceans_add() -> Html {
    let current_user_context =
        use_context::<CurrentUserContext>().expect("Current user context is missing.");

    match &current_user_context.token {
        Some(token) => {
            let cloned_token = token.clone();
            spawn_local(async move {
                api_rustaceans(&cloned_token).await;
            });

            html! {
                <div class="container">
                    <div class="row">
                        <div class="col-sm-auto">
                            <Sidebar />
                        </div>
                        <div class="col mt-3">
                            <Header />
                            <RustaceanForm />
                        </div>
                    </div>
                </div>
            }
        }
        None => {
            html! {
                <Redirect<Route> to={Route::Login} />
            }
        }
    }
}
