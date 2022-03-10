use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="app">
            <header class="app-header">
                <p>
                    {"Page does not exist."}
                </p>
            </header>
        </div>
    }
}