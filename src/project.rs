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
}

pub struct Project {
    name: String,
    path: String,
}

impl Project {
    pub fn new(name: String, path: String) -> Self {
        Project { name, path }
    }
}