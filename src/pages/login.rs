use yew::{function_component, Html, html};

use crate::components::input::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class="container">
            <div class="row min-vh-100 justify-content-center align-items-center">
                <div div="col-md-4">
                    <form>
                        <div class="mb-3">
                            <Input input_type="text" name="ユーザ名" label="ユーザ名" />
                        </div>
                        <div class="mb-3">
                            <Input input_type="password" name="パスワード" label="パスワード" />
                        </div>
                        <button type="submit">{"ログイン"}</button>
                    </form>
                </div>
            </div>
        </div>
    }
}
