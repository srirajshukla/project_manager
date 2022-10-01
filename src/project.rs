#[derive(Debug)]
pub struct ProjectHolder {
    pub name: String,
    pub projects: Vec<Project>
}

impl ProjectHolder {
    pub fn new(name: String) -> Self {
        ProjectHolder {
            name,
            projects: vec![]
        }
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }
}

#[derive(Debug)]
pub struct Project {
    name: String,
    path: String,
}

impl Project {
    pub fn new(name: &str, path: &str) -> Self {
        Project { name: String::from(name) , path: String::from(path) }
    }
}