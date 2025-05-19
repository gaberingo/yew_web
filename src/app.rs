use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h1 class="border-dashed">{ "Hello, Yew!" }</h1>
            <p>{ "This is a simple Yew application." }</p>
        </div>
    }
}