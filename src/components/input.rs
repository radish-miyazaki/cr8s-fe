use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,
    pub input_type: AttrValue,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let html_id = format!("edit-{}", props.name);
    html! {
        <label id={html_id.clone()}>{props.label.clone()}
            <input
                id={html_id}
                type={props.input_type.clone()}
                name={props.name.clone()}
            />
        </label>
    }
}
