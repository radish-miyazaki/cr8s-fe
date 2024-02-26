use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,
    pub input_type: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let html_id = format!("edit-{}", props.name);
    html! {
        <label id={html_id.clone()}>{props.label.clone()}
            <input
                id={html_id}
                class="form-control"
                type={props.input_type.clone()}
                name={props.name.clone()}
                value={props.value.clone()}
                onchange={props.onchange.clone()}
            />
        </label>
    }
}
