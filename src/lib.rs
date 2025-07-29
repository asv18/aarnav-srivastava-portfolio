use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use leptos_use::{use_window_size, UseWindowSizeReturn};

// Modules
mod components;
mod pages;
mod utils;

// Top-Level pages
use crate::pages::{home::Home, not_found::NotFound};

/// An app router which renders the homepage and handles 404's
#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="dark" />

        // sets the document title
        <Title text="Aarnav Srivastava | Portfolio" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=Home />
            </Routes>
        </Router>
    }
}
