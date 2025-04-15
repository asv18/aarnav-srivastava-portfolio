use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};

use leptos::prelude::*;
use leptos::wasm_bindgen::{prelude::Closure, JsCast};

#[allow(non_snake_case)]
#[component]
pub fn Heading() -> impl IntoView {
    let (name, set_name) = signal("".to_string());

    Effect::new(move |_| {
        typewriter(30.0,  "Aarnav Srivastava", name, set_name)
    });

    view! {
        <div class="heading">
            // <img src="/images/larry.jpeg" alt="Profile Image" class="profile" />
            <h1 class="typewriter">{name}</h1>
        </div>
    }
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
    per_char_duration: f64,
    target: &str,
    current_text: ReadSignal<String>,
    signal: WriteSignal<String>,
) -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut start_time: Option<f64> = None;
    let mut target = VecDeque::from(Vec::from(target));

    *g.borrow_mut() = Some(Closure::new(move || {
        if target.is_empty() {
            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        let timestamp = Date::now();

        let start = start_time.get_or_insert(timestamp);
        let elapsed = timestamp - *start;
        let progress = (elapsed / per_char_duration).min(1.0);

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
