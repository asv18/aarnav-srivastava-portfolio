use leptos::{html::Div, prelude::*};
use leptos_use::use_element_visibility;
use crate::{components::{animated_components::typewriter::Typewriter, project_container::ProjectContainer}, utils::types::project::Project};

#[allow(non_snake_case)]
#[component]
pub fn Body() -> impl IntoView {
    let (toggle, set_toggle) = signal(false);

    // let titles = vec!["Software Developer", "Researcher", "Physicist", "Computer Scientist"];
    let projects = vec![
        Project::from_str("Name:\nCircles\nDescription:\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent fermentum felis a quam tincidunt, sit amet fermentum massa tempus. Aliquam erat volutpat. Curabitur vel augue nec velit vehicula luctus. Morbi euismod orci at dolor efficitur, at dignissim leo dictum.\nSuspendisse potenti. Vivamus faucibus tincidunt nisl at lobortis. Sed fringilla, lacus ut sodales efficitur, mi nulla feugiat neque, non imperdiet nunc ipsum sed elit. Donec vel lacus nec leo vulputate viverra nec et elit. Proin id porttitor velit, sed ultrices lorem.\nNullam eu nulla nec sapien viverra eleifend. Pellentesque porttitor lorem ut ex efficitur, at rhoncus nisi tincidunt. Nunc feugiat diam vitae arcu porttitor, nec malesuada sapien fermentum. Integer ultricies purus eget dapibus tempor. Duis eu sapien nec metus ultricies pulvinar.\nTech Stack:\nFlutter, SQL, Deno\nLink:\nhttps://github.com/asv18/Circles-App").unwrap_or_default(),
    ];
    // let projects = vec!["Circles", "CampusConnect", "Josephson Junctions", "MITRA"];

    let projects_div = NodeRef::<Div>::new();
    let projects_vis = use_element_visibility(projects_div);

    view! {
        <div class="body">
            <div class="child">
                <div class="heading">
                    // <img src="/images/larry.jpeg" alt="Profile Image" class="profile" />
                    <Typewriter
                        target_name="Hi! I'm Aarnav Srivastava!"
                        per_char_duration=50.0
                        set_toggle
                    />
                </div>
                <div class=move || {
                    format!("desc {} ", if toggle.get() { "content" } else { "content-hidden" })
                }>
                    // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras ultricies diam id metus mollis, eu feugiat dui congue. Ut mattis pretium velit, a convallis nulla pretium at. Maecenas quis quam porta, blandit nulla vel, laoreet lacus. Nunc mauris lacus, ultricies vel purus at, condimentum convallis ex. Quisque arcu felis, egestas id urna at, vulputate efficitur nunc. Morbi tincidunt enim a augue tristique, quis auctor erat posuere. Donec nec leo ut nunc scelerisque dignissim at a lorem. Vestibulum rhoncus, nisl id ultricies dictum, velit risus scelerisque leo, at aliquam lorem augue ac felis. In sed velit non leo ullamcorper interdum suscipit non lacus. Fusce in consequat neque, vel pellentesque ligula. Phasellus congue quis lorem nec ultricies. Nullam faucibus ante quam, condimentum fermentum ex dignissim in.
                    // <h2 class="heading-2">"And I'm an aspiring..."</h2>
                    // <ul>{titles.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ul>
                    // <h2 class="heading-2">"at UIUC."</h2>
                    <p>
                        "I'm an aspiring physicist, software developer, and researcher studying Computer Science + Physics at University of Illinois Urbana-Champaign."
                    </p>
                    <p>
                        I have experience with ML design, language-processing, and quantum computing.
                    </p>
                    <p>
                        I am interested on working projects in the field of quantum information science and superconducting theory.
                    </p>
                    <p>
                        "Currently, I'm working with the Bezryadin lab at UIUC on an AI model to analyze and optimize geometries for arrays of Josephson Junctions."
                    </p>
                </div>
            </div>
            <div class="child center">
                <div
                    node_ref=projects_div
                    class=move || {
                        format!(
                            " {} ",
                            if projects_vis.get() && toggle.get() {
                                "delay content"
                            } else {
                                "content-hidden"
                            },
                        )
                    }
                >
                    <h1 class="heading-2">"My work"</h1>
                    <div class="column">
                        {projects
                            .iter()
                            .map(|item| {
                                view! {
                                    <div>
                                        <ProjectContainer project=item.clone() />
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                    <h2 class="heading-2">"...and more!"</h2>
                </div>
            </div>
        </div>
    }
}