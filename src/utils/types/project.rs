#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub link: String,
}

impl Default for Project {
    fn default() -> Self {
        Self { name: "Unknown".to_string(), description: "Unknown".to_string(), tech_stack: Default::default(), link: "https://github.com/asv18".to_string() }
    }
}

impl Project {
    pub fn from_str(contents: &str) -> std::io::Result<Self> {
        let mut name = String::new();
        let mut description = String::new();
        let mut tech_stack = Vec::new();
        let mut link = String::new();

        let mut current_section = Section::None;

        for line in contents.lines() {
            match line.trim() {
                "Name:" => current_section = Section::Name,
                "Description:" => current_section = Section::Description,
                "Tech Stack:" => current_section = Section::TechStack,
                "Link:" => current_section = Section::Link,
                _ => {
                    match current_section {
                        Section::None => continue,
                        Section::Name => name.push_str(&line),
                        Section::Description => description.push_str(&line),
                        Section::TechStack => {
                            let techs = line.split(",").into_iter();

                            for tech in techs {
                                if !tech.is_empty() {
                                    tech_stack.push(tech.trim().to_string());
                                }
                            }
                        },
                        Section::Link => link.push_str(&line),
                    }
                }
            }
        }


        Ok(
            Project { name, description, tech_stack, link }
        )
    }
}

enum Section {
    None,
    Name,
    Description,
    TechStack,
    Link,
}