use leptos::prelude::*;
use crate::components::animated_components::typewriter::Typewriter;

#[allow(non_snake_case)]
#[component]
pub fn Body() -> impl IntoView {
    let (toggle, set_toggle) = signal(false);

    view! {
        <div class="body">
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
                "I'm an aspiring"<br />"Software Developer"<br />"Researcher"<br />"Physicist"<br />
                "Computer Scientist"
            </div>
        </div>
    }
}