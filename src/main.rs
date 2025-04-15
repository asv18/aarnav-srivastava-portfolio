use leptos::prelude::*;
use portfolio_website::App;

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            // 
            <div class="root">
                <App />
            </div>
        }
    })
}
