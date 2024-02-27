use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: AttrValue,
    pub alert_type: AttrValue,
}

#[function_component(Alert)]
pub fn alert(props: &Props) -> Html {
    html! {
        <div class={format!("alert alert-{}", props.alert_type.clone())} role="alert">
            {props.message.clone()}
        </div>
    }
}
