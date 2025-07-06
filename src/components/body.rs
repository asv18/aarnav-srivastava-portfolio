use leptos::{html::Div, prelude::*};
use leptos_use::use_element_visibility;
use crate::components::animated_components::typewriter::Typewriter;

#[allow(non_snake_case)]
#[component]
pub fn Body() -> impl IntoView {
    let (toggle, set_toggle) = signal(false);

    // let titles = vec!["Software Developer", "Researcher", "Physicist", "Computer Scientist"];
    let projects = vec!["Circles", "CampusConnect", "Josephson Junctions", "MITRA"];

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
                    format!("para {} ", if toggle.get() { "content" } else { "content-hidden" })
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
                    <h2 class="heading-2">"Some of my projects"</h2>
                    <ul>{projects.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ul>
                    <h2 class="heading-2">"...and more!"</h2>
                </div>
            </div>
        </div>
    }
}