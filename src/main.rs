use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use project_lib::project::get_projects_in_path;




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
