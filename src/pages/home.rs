// use crate::components::{counter_btn::Button, practice::Practice, iteration::Iteration, forms::Forms};
use leptos::prelude::*;

use crate::components::body::Body;

/// Default Home Page
#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="scaffold">
                <Body />
            </div>
        </ErrorBoundary>
    }
}
