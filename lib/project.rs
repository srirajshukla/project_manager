use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use walkdir::{DirEntry, WalkDir};


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


fn is_blacklisted_project(entry: &DirEntry, ignore_dir: &Vec<&str>) -> bool {
    // println!("{:?}", entry.file_name().to_str());
    entry
        .file_name()
        .to_str()
        .map(|s| ignore_dir.contains(&s))
        .unwrap_or(false)
}

fn is_project_descriptor(filename: &str, project_files: &Vec<&str>) -> bool {
    project_files.contains(&filename)
}

pub fn get_projects_in_path(path: &str, ignore_dirs: &Vec<&str>, project_files: &Vec<&str>)-> Vec<Project> {
    let mut project_dirs: Vec<DirEntry> = Vec::new();

    let walker = WalkDir::new(path).into_iter();
    for entry in walker
        .filter_entry(|e| !is_blacklisted_project(e, &ignore_dirs))
        .filter(|e| {
            e.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .map(|s| is_project_descriptor(s, &project_files))
                .unwrap_or(false)
        })
    {
        println!("{}", &entry.as_ref().unwrap().path().display());
        let e = entry.unwrap().clone();
        project_dirs.push(e);
    }


    project_dirs.iter().map(|e| {
        let ev = e.path().to_owned();
        let mut p = Project::new(ev.parent().unwrap().to_str().unwrap(), ev.to_str().unwrap());
        p.set_project_type(ev.file_name().unwrap().to_str().unwrap());
        p
    }).collect()

}

fn project_root_projects(dirs : &Vec<String>, root : &str){

    let mut root_dirs = HashMap::<&str, ProjectHolder>::new();

    for dir in dirs {
        let suffix = dir.strip_prefix(root).unwrap();
        
        if root_dirs.contains_key(suffix) {

            
            root_dirs.get_mut(suffix).unwrap().add_project(Project::new(suffix, dir));
            // project_holder = root_dirs.get(suffix).unwrap().clone();
        } else {
            let mut project_holder = ProjectHolder::new(suffix.to_string());
            project_holder.add_project(Project::new(suffix, dir));
            root_dirs.insert(suffix, project_holder);
        }

    }

    println!("{:#?}", root_dirs);
}