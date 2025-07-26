use leptos::{html::Div, prelude::*};
use leptos_use::use_element_visibility;
use crate::{components::{animated_components::typewriter::Typewriter, project_container::ProjectContainer}, utils::types::project::Project};

#[allow(non_snake_case)]
#[component]
pub fn Body() -> impl IntoView {
    let (toggle, set_toggle) = signal(false);

    // let titles = vec!["Software Developer", "Researcher", "Physicist", "Computer Scientist"];
    let projects = vec![
        Project::from_str("Name:\nJosephson Junctions\nDescription:\nDeveloping an optimization tool utilizing a variety of stochastic searches, including Bayesian Optimization, Genetic Algorithms, and Gradient Descent to determine the ideal geometry of n-Josephson Junctions in accordance with current experimental data.\nTech Stack:\nPython, C++\nLink:\nhttps://github.com/asv18/Circles-App").unwrap_or_default(),
        Project::from_str("Name:\nMITRA Project\nDescription:\nUsed Python scripts and prompt engineering to deduplicate data, used Rust and fasttext word embeddings to identify the language a file is written in, labeled data, and cleaned large (~30k+ characters) data sources. Work was presented at the International Sanskrit Computational Linguistics Symposium.\nTech Stack:\nRust, Python, PyTorch\nLink:\nhttps://github.com/dharmamitra/sanskrit-english-identification").unwrap_or_default(),
        Project::from_str("Name:\nCircles\nDescription:\nUsed Flutter, oak/Deno, CockroachDB, and Firebase to develop a social goal setting application to allow users to set and track goals visible through live feeds, connect with like-minded individuals via messaging, and foster communities around shared interests.\nTech Stack:\nFlutter, SQL, Deno\nLink:\nhttps://github.com/asv18/Circles-App").unwrap_or_default(),
    ];
    // let projects = vec!["Circles", "CampusConnect", "Josephson Junctions", "MITRA"];

    let projects_div = NodeRef::<Div>::new();
    let projects_vis = use_element_visibility(projects_div);

    let education_div = NodeRef::<Div>::new();
    let education_vis = use_element_visibility(education_div);

    let contacts_div = NodeRef::<Div>::new();
    let contacts_vis = use_element_visibility(contacts_div);

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
                        "I have experience with ML design, language-processing, and quantum computing."
                    </p>
                    <p>
                        "I am interested on working projects in the field of quantum information science and superconducting theory."
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
                    <h1>"My work"</h1>
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
                    <a
                        href="https://github.com/asv18/"
                        target="_blank"
                        class="heading-2 project-link"
                    >
                        <h2>"...and more!"</h2>
                    </a>
                </div>
            </div>
            <div class="child center">
                <div
                    node_ref=education_div
                    class=move || {
                        format!(
                            " {} ",
                            if education_vis.get() && toggle.get() {
                                "delay content"
                            } else {
                                "content-hidden"
                            },
                        )
                    }
                >
                    <h1>"My education"</h1>
                    <div class="spaced-title-row">
                        <div class="title">"University of Illinois, Urbana-Champaign"</div>
                        <div>"B.S. in Computer Science + Physics"</div>
                    </div>
                    <div class="breakline" />
                    <div class="column">
                        <div class="spaced-row">
                            <div>"GPA: 3.9/4.0"</div>
                            <div>"AUG 2024-MAY 2028"</div>
                        </div>
                        <div>"Honors: James Scholar, Dean's List"</div>
                        <div>
                            "Relevant courses: Quantum Physics, Intro to Quantum Info Theory and Comp, Computer Architecture, Abstract Linear Algebra, Data Structures, Discrete Structures, Intro to Differential Equations, Relativity and Math Applications, Intro to Computational Physics, Intro to CS I & II"
                        </div>
                    </div>
                </div>
            </div>
            <div class="child center">
                <div
                    node_ref=contacts_div
                    class=move || {
                        format!(
                            " {} ",
                            if contacts_vis.get() && toggle.get() {
                                "delay content"
                            } else {
                                "content-hidden"
                            },
                        )
                    }
                >
                    <h1>"My contacts"</h1>
                    <div class="column">
                        <a
                            href="mailto:aarnav.srivastava18@gmail.com"
                            target="_blank"
                            class="element element-link"
                        >
                            <div>"aarnav.srivastava18@gmail.com"</div>
                        </a>
                        <div class="breakline" />
                        <a
                            href="mailto:aarnavs2@illinois.edu"
                            target="_blank"
                            class="element element-link"
                        >
                            <div>"aarnavs2@illinois.edu"</div>
                        </a>
                        <div class="breakline" />
                        <a
                            href="https://github.com/asv18?tab=repositories"
                            target="_blank"
                            class="element element-link"
                        >
                            <div>"GitHub"</div>
                        </a>
                        <div class="breakline" />
                        <a
                            href="https://www.linkedin.com/in/aarnav-srivastava-5aba35240/"
                            target="_blank"
                            class="element element-link"
                        >
                            <div>"LinkedIn"</div>
                        </a>
                        <div class="breakline" />
                    // <div>"Link to resume"</div>
                    // <div class="breakline" />
                    </div>
                </div>
            </div>
        </div>
    }
}