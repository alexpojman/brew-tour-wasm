use yew::prelude::*;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="app">
            <header class="app-header">
                <p>{"Welcome to Brewery Tour 🍺"}</p>
            </header>
        </div>
    }
}
