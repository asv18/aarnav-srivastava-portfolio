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
                <a
                    href=project.link
                    target="_blank"
                    class=format!(
                        "title title-font proj-title {}",
                        if project.link.is_empty() { "inactive-link" } else { "proj-title-link" },
                    )
                >
                    {project.name}
                </a>
                // <div class="proj-spacer"></div>
                <div class="proj-container-stack">
                    {project
                        .tech_stack
                        .into_iter()
                        .map(|item| {
                            view! {
                                <div class=format!(
                                    "boxed {}",
                                    item.to_lowercase(),
                                )>{item.clone()}</div>
                            }
                        })
                        .collect_view()}
                </div>
                <div class="para proj-container-hidden">{project.description}</div>
            // <div>{project.image}</div>
            </div>
        // <div>"Cube"</div>
        </div>
    }
}