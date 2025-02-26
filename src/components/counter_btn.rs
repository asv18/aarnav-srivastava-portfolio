use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn Button(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let (count, set_count) = signal(0);
    
    // let increment_count = move || count.get() + increment;
    view! {
        <button
            on:click=move |_| *set_count.write() += increment
            // set the `style`
            // class:red=move || count.get() % 2 == 1
            // style="position: absolute"
            // and toggle individual CSS properties with `style:`
            // style:transition
            // style:left=move || format!("{}px", count.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", move || count.get().to_string())
        >
            "Click me: "
            {count}
        </button>
    }
}
