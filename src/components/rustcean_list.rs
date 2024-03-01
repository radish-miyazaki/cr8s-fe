use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::rustaceans::api_rustaceans;
use crate::contexts::CurrentUserContext;
use crate::Route;

#[function_component(RustceanList)]
pub fn rustcean_list() -> Html {
    let current_user_context =
        use_context::<CurrentUserContext>().expect("Current user context is missing.");

    let rustacenas_handle = use_state(Vec::new);
    let rustaceans = (*rustacenas_handle).clone();

    match &current_user_context.token {
        Some(token) => {
            let cloned_token = token.clone();

            spawn_local(async move {
                let response = api_rustaceans(&cloned_token).await.unwrap();
                rustacenas_handle.set(response);
            });

            html! {
                <>
                    <p>
                        <Link<Route> to={Route::RustaceansAdd}>
                            {"+ Add new rustacean"}
                        </Link<Route>>
                    </p>
                    <table class="table">
                        <thead>
                            <th>{"ID"}</th>
                            <th>{"Name"}</th>
                            <th>{"Email"}</th>
                            <th>{"Created at"}</th>
                            <th>{"Operations"}</th>
                        </thead>
                        <tbody>
                            {
                                rustaceans.into_iter().map(|rustacean| {
                                    html! {
                                        <tr>
                                            <td>{rustacean.id}</td>
                                            <td>{rustacean.name}</td>
                                            <td>{rustacean.email}</td>
                                            <td>{rustacean.created_at}</td>
                                            <td>
                                                <Link<Route> to={Route::RustaceansAdd}>
                                                    {"Edit"}
                                                </Link<Route>>
                                                <span> {"/"} </span>
                                                <Link<Route> to={Route::RustaceansAdd}>
                                                    {"Delete"}
                                                </Link<Route>>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                </>
            }
        }
        None => {
            html! {
                <Redirect<Route> to={Route::Login} />
            }
        }
    }
}
