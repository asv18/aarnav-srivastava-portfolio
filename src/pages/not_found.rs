use leptos::prelude::*;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="scaffold">
            <h1>"Uh oh!"</h1>
            <div>"We couldn't find that page!"</div>
        </div>
    }
}
