use serde::{Serialize, Deserialize};

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

    pub fn get_projects(&self) -> &Vec<Project> {
        &self.projects
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    path: String,
    project_type: String
}

impl Project {
    pub fn new(name: &str, path: &str) -> Self {
        Project { name: String::from(name) , path: String::from(path) , project_type: String::from("") }
    }

    pub fn new_with_type(name: &str, path: &str, project_type: &str) -> Self {
        Project { name: String::from(name) , path: String::from(path) , project_type: String::from(project_type) }
    }

    pub fn set_project_type(&mut self, project_type: &str) {
        self.project_type = String::from(project_type);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}
