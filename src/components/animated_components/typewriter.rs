use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};

use leptos::prelude::*;
use leptos::wasm_bindgen::{prelude::Closure, JsCast};

#[allow(non_snake_case, dead_code)]
#[component]
pub fn Typewriter(
    target_name: &'static str,
    per_char_duration: f64,
    set_toggle: WriteSignal<bool>
) -> impl IntoView {
    let (name, set_name) = signal("".to_string());

    Effect::new(move |_| {
        typewriter(per_char_duration,  target_name, name, set_name, set_toggle)
    });

    view! { <h1 class="typewriter">{name}</h1> }
}

use leptos::wasm_bindgen::prelude::*;
use leptos::web_sys::{Window, window, js_sys::Date};

fn get_window() -> Window {
    window().expect("no global 'window' exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    get_window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn typewriter(
    duration: f64,
    target: &'static str,
    current_text: ReadSignal<String>,
    signal: WriteSignal<String>,
    signal_end: WriteSignal<bool>,
) -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut start_time: Option<f64> = None;

    let mut target = target.chars().into_iter().collect::<VecDeque<char>>();

    *g.borrow_mut() = Some(Closure::new(move || {
        if target.is_empty() {
            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            signal_end.set(true);

            return;
        }

        let timestamp = Date::now();

        let start = start_time.get_or_insert(timestamp);
        let elapsed = timestamp - *start;
        let progress = (elapsed / duration).min(1.0);

        // Set the body's text content to the next character based on time elapsed
        // requestAnimationFrame callback has fired.

        if progress >= 1.0 {
            let next_char = target.pop_front().unwrap() as char;
            signal.set(format!("{}{}", current_text.get(), next_char));
            start_time = None; // reset timer for next character
        }

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
