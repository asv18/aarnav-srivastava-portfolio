use leptos::prelude::*;

use crate::utils::types::project::Project;

#[allow(non_snake_case)]
#[component]
pub fn ProjectContainer(
    project: Project,
) -> impl IntoView {
    view! {
        <div class="row">
            <div class="proj-container content">
                <div class="title">{project.name}</div>
                // <div class="proj-spacer"></div>
                <div class="proj-container-stack"></div>
                <div class="para proj-container-hidden">{project.description}</div>
            </div>
            <div>"Cube"</div>
        </div>
    }
}