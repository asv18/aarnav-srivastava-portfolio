use std::{marker::PhantomData, time::Duration};

use leptos::prelude::*;

#[component]
pub fn Practice() -> impl IntoView {
    let (count, set_count) = signal(0);

    // Create a signal to track whether the button is clicked
    let (clicked, set_clicked) = signal(false);

    // Reset the animation after a delay
    let reset_animation = move || {
        set_clicked.set(true);
        set_timeout(move || set_clicked.set(false), Duration::from_millis(200));
    };

    let injectable_html = r#"<p class="text-20">"This HTML is injected!"</p>"#;

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
                reset_animation();
            }
            class=move || {
                format!("button-20 text-20 {} ", if clicked.get() { "clicked" } else { "" })
            }
        >
            // class=""
            "Click This!!: "
            {count}
        </button>
        <ProgressBar
            max=50u16
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            progress=count
        />
        <ProgressBar
            max=50u16
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            progress=Signal::derive(move || count.get() * 2)
        />
        // example of generics
        <SizeOf<usize> />
        <div inner_html=injectable_html />
    }
}

#[component]
fn ProgressBar(
    #[prop(default = 20)]
    max: u16,
    // need to signal passing of reactice element; Signal type allows for generics
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress max=max value=progress />
        <br />
    }
}

// note you cant specify optional generics:
/*
 *     #[prop(optional)] progress: Option<F>,
*/
// we can get around this using Box

#[component]
fn SizeOf<T: Sized>(#[prop(optional)] _ty: PhantomData<T>) -> impl IntoView {
    std::mem::size_of::<T>()
}