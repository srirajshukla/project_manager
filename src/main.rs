pub mod project;

use std::collections::HashMap;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use project::ProjectHolder;
use project::Project;
use walkdir::{DirEntry, WalkDir};


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

fn get_projects_in_path(path: &str, ignore_dirs: &Vec<&str>, project_files: &Vec<&str>)-> Vec<Project> {
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

async fn res() -> impl Responder{
    let path = "D:\\workspace\\rust";
    let ignore_dirs = vec!["node_modules", "target", ".git", "dist", "build"];
    let project_files = vec!["Cargo.toml", "package.json", "CMakeLists.txt", "pom.xml", ];

    let projects =  get_projects_in_path(&path, &ignore_dirs, &project_files);
    // x(&projects, &path);

    HttpResponse::Ok().json(projects)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    // res().await;

    // return Ok(());

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::permissive()
            )
            .route("/hello", web::get().to(res))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_blacklisted_project() {
        let ignore_dirs = vec!["node_modules", "target", ".git", "dist", "build"];
        // let entry = DirEntry::new("D:\\workspace\\rust\\src\\main.rs");
        // assert_eq!(is_blacklisted_project(&entry, &ignore_dirs), false);
    }
}