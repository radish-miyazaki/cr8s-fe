use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::{
    api::{
        rustaceans::api_rustacean_create,
        user::{api_login, api_me, LoginResponse, MeResponse},
    },
    components::{alert::Alert, input::Input},
    contexts::CurrentUserContext,
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

#[function_component(RustaceanForm)]
pub fn rustacean_form() -> Html {
    let navigator = use_navigator().expect("Navigator not available.");
    let current_user_context =
        use_context::<CurrentUserContext>().expect("Current user context is missing.");

    let name_handle = use_state(String::default);
    let name = (*name_handle).clone();
    let name_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            name_handle.set(input.value());
        }
    });

    let email_handle = use_state(String::default);
    let email = (*email_handle).clone();
    let email_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            email_handle.set(input.value());
        }
    });

    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    let cloned_name = name.clone();
    let cloned_email = email.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();

        let cloned_name = cloned_name.clone();
        let cloned_email = cloned_email.clone();
        let cloned_error_handle = error_message_handle.clone();
        let cloned_navigator = navigator.clone();
        let cloned_current_user_context = current_user_context.clone();

        match &cloned_current_user_context.token {
            Some(token) => {
                let cloned_token = token.clone();
                spawn_local(async move {
                    match api_rustacean_create(
                        &cloned_token,
                        cloned_name.clone(),
                        cloned_email.clone(),
                    )
                    .await
                    {
                        Ok(rustacean) => {
                            cloned_navigator.push(&Route::Rustaceans);
                        }
                        Err(e) => cloned_error_handle.set(e.to_string()),
                    }
                });
            }
            None => cloned_error_handle.set("Session expired. Please login again".to_string()),
        }
        spawn_local(async move {})
    });

    html! {
        <form onsubmit={onsubmit}>
            if !error_message.is_empty() {
                <Alert message={error_message} alert_type={"danger"} />
            }
            <div class="mb-3">
                <Input
                    input_type="text"
                    name="名前"
                    label="名前"
                    value={name}
                    onchange={name_changed}
                />
            </div>
            <div class="mb-3">
                <Input
                    input_type="email"
                    name="メールアドレス"
                    label="メールアドレス"
                    value={email}
                    onchange={email_changed}
                />
            </div>
            <button
                type="submit"
                class="btn btn-primary"
            >
                {"保存"}
            </button>
        </form>
    }
}
