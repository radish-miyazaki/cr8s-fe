use yew::prelude::*;
use yew_router::components::Redirect;

use crate::{components::login_form::*, contexts::CurrentUserContext, Route};

#[function_component(Login)]
pub fn login() -> Html {
    let current_user_context =
        use_context::<CurrentUserContext>().expect("Current user context is missing.");

    match &current_user_context.user {
        Some(_) => {
            html! {
                <Redirect<Route> to={Route::Home} />
            }
        }
        None => {
            html! {
                <div class="container">
                    <div class="row min-vh-100 justify-content-center align-items-center">
                        <div div="col-md-4">
                            <LoginForm />
                        </div>
                    </div>
                </div>
            }
        }
    }
}
