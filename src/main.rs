use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::process::Command;
use std::collections::HashMap;

async fn whois(info: web::Query<HashMap<String, String>>) -> impl Responder {
    if let Some(ip) = info.get("ip") {
        let output = Command::new("whois").arg(ip).output();
        match output {
            Ok(out) => {
                let raw_output = String::from_utf8_lossy(&out.stdout);
                let mut parsed_data = HashMap::new();

                let keys = [
                    "inetnum", "netname", "descr", "country", "admin-c", "tech-c",
                    "status", "remarks", "mnt-by", "mnt-irt", "mnt-lower", "mnt-routes",
                    "last-modified", "source"
                ];

                for line in raw_output.lines() {
                    if let Some((key, value)) = line.split_once(':') {
                        let key = key.trim();
                        let value = value.trim();
                        if keys.contains(&key) {
                            parsed_data.insert(key.to_string(), value.to_string());
                        }
                    }
                }

                HttpResponse::Ok().json(parsed_data) // Mengembalikan dalam format JSON
            }
            Err(_) => HttpResponse::InternalServerError().body("Error executing whois command"),
        }
    } else {
        HttpResponse::BadRequest().body("IP address is required")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/whois", web::get().to(whois)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
