use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::{
    api::user::{api_login, api_me, LoginResponse, MeResponse},
    components::{alert::Alert, input::Input},
    Route,
};

async fn login(
    username: String,
    password: String,
) -> Result<(LoginResponse, MeResponse), gloo_net::Error> {
    let login_resposne = api_login(username, password).await?;
    let me_response = api_me(&login_resposne.token).await?;

    Ok((login_resposne, me_response))
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let navigator = use_navigator();

    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();
    let username_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            username_handle.set(input.value());
        }
    });

    let password_handle = use_state(String::default);
    let password = (*password_handle).clone();
    let password_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            password_handle.set(input.value());
        }
    });

    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    let cloned_username = username.clone();
    let cloned_password = password.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let cloned_username = cloned_username.clone();
        let cloned_password = cloned_password.clone();
        let cloned_error_handle = error_message_handle.clone();
        let cloned_navigator = navigator.clone();
        spawn_local(async move {
            match login(cloned_username.clone(), cloned_password.clone()).await {
                Ok(_responses) => {
                    if let Some(nav) = cloned_navigator {
                        nav.push(&Route::Home);
                    }
                }
                Err(e) => cloned_error_handle.set(e.to_string()),
            }
        })
    });

    html! {
        <form onsubmit={onsubmit}>
            if !error_message.is_empty() {
                <Alert message={error_message} alert_type={"danger"} />
            }
            <div class="mb-3">
                <Input
                    input_type="text"
                    name="ユーザ名"
                    label="ユーザ名"
                    value={username}
                    onchange={username_changed}
                />
            </div>
            <div class="mb-3">
                <Input
                    input_type="password"
                    name="パスワード"
                    label="パスワード"
                    value={password}
                    onchange={password_changed}
                />
            </div>
            <button
                type="submit"
                class="btn btn-primary"
            >
                {"ログイン"}
            </button>
        </form>
    }
}
