use yew::prelude::*;

use crate::components::login_form::*;

#[function_component(Login)]
pub fn login() -> Html {
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
