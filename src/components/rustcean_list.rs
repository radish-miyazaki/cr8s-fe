use yew::prelude::*;
use yew_router::prelude::*;

use crate::hooks::use_rustaceans;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token: AttrValue,
}

#[function_component(RustceanList)]
pub fn rustcean_list(props: &Props) -> HtmlResult {
    let rustaceans = use_rustaceans(&props.token)?;

    Ok(html! {
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
    })
}
