use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub image: String,
}

impl Default for Project {
    fn default() -> Self {
        Self { name: "Unknown".to_string(), description: "Unknown".to_string(), tech_stack: Default::default(), image: Default::default() }
    }
}

impl Project {
    pub fn from_str(contents: &str) -> std::io::Result<Self> {
        let mut name = String::new();
        let mut description = String::new();
        let mut tech_stack: Vec<String> = Vec::new();
        let mut image = String::new();

        let mut current_section = Section::None;

        for line in contents.lines() {
            match line.trim() {
                "Name:" => current_section = Section::Name,
                "Description:" => current_section = Section::Description,
                "Tech Stack:" => current_section = Section::TechStack,
                "Image:" => current_section = Section::Image,
                _ => {
                    match current_section {
                        Section::None => continue,
                        Section::Name => name.push_str(&line),
                        Section::Description => description.push_str(&line),
                        Section::TechStack => {
                            let techs = line.split(",").into_iter();

                            for tech in techs {
                                tech_stack.push(tech.to_string());
                            }
                        },
                        Section::Image => image.push_str(&line),
                    }
                }
            }
        }


        Ok(
            Project { name, description, tech_stack, image }
        )
    }
}

enum Section {
    None,
    Name,
    Description,
    TechStack,
    Image,
}