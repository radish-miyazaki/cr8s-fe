use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    contexts::{CurrentDispatchAction, CurrentUserAction, CurrentUserContext},
    Route,
};

#[function_component(Header)]
pub fn header() -> Html {
    let current_user_context =
        use_context::<CurrentUserContext>().expect("Current user context is missing.");

    match &current_user_context.user {
        Some(user) => {
            let cloned_current_user_context = current_user_context.clone();
            let onclick = Callback::from(move |e: MouseEvent| {
                e.prevent_default();

                cloned_current_user_context.dispatch(CurrentDispatchAction {
                    action_type: CurrentUserAction::LoginFail,
                    login_response: None,
                    me_response: None,
                })
            });

            html! {
                <div class="text-end">
                    <p>
                        <span class="pe-1">{format!("Welcome {}", user.username.clone())}</span>
                        <button class="btn btn-danger" onclick={onclick}>
                            {"Logout"}
                        </button>
                    </p>
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
