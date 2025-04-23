use leptos::prelude::*;
use leptos::web_sys::Element;
use crate::components::animated_components::typewriter::Typewriter;

#[allow(non_snake_case)]
#[component]
pub fn Body() -> impl IntoView {
    let (toggle, set_toggle) = signal(false);

    let titles = vec!["Software Developer", "Researcher", "Physicist", "Computer Scientist"];

    view! {
        <div class="body">
            <div class="child">
                <div class="heading">
                    <img src="/images/larry.jpeg" alt="Profile Image" class="profile" />
                    <Typewriter
                        target_name="Hi! I'm Aarnav Srivastava!"
                        per_char_duration=50.0
                        set_toggle
                    />
                </div>
                <div class=move || {
                    format!(" {} ", if toggle.get() { "content" } else { "content-hidden" })
                }>
                    <h2 class="heading-2">"And I'm an aspiring..."</h2>
                    // "Software Developer"
                    // <br />
                    // "Researcher"
                    // <br />
                    // "Physicist"
                    // <br />
                    // "Computer Scientist"
                    <ul>{titles.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ul>
                    <h2 class="heading-2">"at UIUC."</h2>
                </div>
            </div>
            <div class="child center">
                <div class=move || {
                    format!(" {} ", if toggle.get() { "content" } else { "content-hidden" })
                }>
                    <h2 class="heading-2">"And I'm an aspiring..."</h2>
                    // "Software Developer"
                    // <br />
                    // "Researcher"
                    // <br />
                    // "Physicist"
                    // <br />
                    // "Computer Scientist"
                    // <ul>{titles.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ul>
                    <h2 class="heading-2">"at UIUC."</h2>
                </div>
            </div>
        </div>
    }
}

fn check_element_visible(element: Element) -> bool {
    let (top, left) = (element.client_top(), element.client_left());

    let window = window();

    window.top()
}