use std::rc::Rc;

use gloo_storage::{SessionStorage, Storage};
use yew::{platform::spawn_local, prelude::*};

use crate::api::user::{api_me, LoginResponse, MeResponse, User};

pub type CurrentUserContext = UseReducerHandle<CurrentUser>;

#[derive(Default, PartialEq)]
pub struct CurrentUser {
    pub user: Option<User>,
    pub token: Option<String>,
}

pub enum CurrentUserAction {
    LoginSuccess,
    LoginFail,
}

pub struct CurrentDispatchAction {
    pub action_type: CurrentUserAction,
    pub login_response: Option<LoginResponse>,
    pub me_response: Option<MeResponse>,
}

impl Reducible for CurrentUser {
    type Action = CurrentDispatchAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let login_response = action.login_response.expect("Missing login response");
        let me_response = action.me_response.expect("Missing me response");
        let _ = SessionStorage::set("cr8s_token", login_response.token.clone());

        match action.action_type {
            CurrentUserAction::LoginSuccess => Self {
                user: Some(User {
                    id: me_response.id,
                    username: me_response.username,
                    created_at: me_response.created_at,
                }),
                token: Some(login_response.token),
            }
            .into(),
            CurrentUserAction::LoginFail => Self {
                user: None,
                token: None,
            }
            .into(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(CurrentUserProvider)]
pub fn current_user_provider(props: &Props) -> Html {
    let user = use_reducer(CurrentUser::default);

    if user.user.is_none() {
        if let Ok(token) = SessionStorage::get::<String>("cr8s_token") {
            let cloned_user = user.clone();

            spawn_local(async move {
                match api_me(&token).await {
                    Ok(me_response) => {
                        cloned_user.dispatch(CurrentDispatchAction {
                            action_type: CurrentUserAction::LoginSuccess,
                            login_response: Some(LoginResponse { token }),
                            me_response: Some(me_response),
                        });
                    }
                    Err(_) => SessionStorage::clear(),
                }
            });
        }
    }

    html! {
        <ContextProvider<CurrentUserContext> context={user}>
            {props.children.clone()}
        </ContextProvider<CurrentUserContext> >
    }
}
