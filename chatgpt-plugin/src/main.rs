use rocket::serde::{json::{Json}};
use rocket::fs::{FileServer, relative};
use rocket::http::Method;
use rocket::post;
use rocket_cors::{AllowedOrigins, AllowedHeaders, CorsOptions};
use std::fs::File;
use std::io::{Write};
use std::process::Command;

#[macro_use] extern crate rocket;

#[derive(serde::Deserialize)]
struct CompileRequest {
    source: String,
}

#[get("/")]
fn index() -> &'static str {
    "Multiversx dev plugin built by github.com/markeljan"
}

#[post("/api/compile", format = "json", data = "<source_code>")]
fn compile(source_code: Json<CompileRequest>) -> String {
    // Path to the contract file
    let file_path = "../deployer/contract/src/contract.rs";

    // Write the new contract code to the file
    match File::create(file_path) {
        Err(why) => {
            return format!("couldn't create {}: {}", file_path, why);
        }
        Ok(mut file) => {
            if let Err(why) = file.write_all(source_code.source.as_bytes()) {
                return format!("couldn't write to {}: {}", file_path, why);
            }
        }
    }

    let output = Command::new("mxpy")
        .current_dir("../deployer/contract")
        .arg("contract")
        .arg("build")
        .output();

    match output {
        Ok(output) => {
            if !output.stderr.is_empty() {
                return String::from_utf8_lossy(&output.stderr).to_string();
            }
            String::from_utf8_lossy(&output.stdout).to_string()
        }
        Err(e) => {
            format!("Failed to execute command: {}", e)
        }
    }
}

#[get("/api/deploy")]
fn deploy() -> String {
    let output = Command::new("mxpy")
        .current_dir("../deployer/contract")
        .arg("contract")
        .arg("deploy")
        .arg("--bytecode")
        .arg("./output/contract.wasm")
        .arg("--pem")
        .arg("../../wallet/wallet-owner.pem")
        .arg("--recall-nonce")
        .arg("--gas-limit")
        .arg("60000000")
        .arg("--chain")
        .arg("D")
        .arg("--proxy")
        .arg("https://devnet-api.multiversx.com")
        .arg("--outfile")
        .arg("deploy-devnet.interaction.json")
        .arg("--send")
        .output();

    match output {
        Ok(output) => {
            if !output.stderr.is_empty() {
                return String::from_utf8_lossy(&output.stderr).to_string();
            }
            String::from_utf8_lossy(&output.stdout).to_string()
        }
        Err(e) => {
            format!("Failed to execute command: {}", e)
        }
    }
}


#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8000", "https://chat.openai.com"]);
    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(
            vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        )
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true)
        .to_cors()
        .unwrap();

    rocket::build()
        .attach(cors)
        .mount("/", routes![index])
        .mount("/", routes![compile])
        .mount("/", routes![deploy])
        .mount("/.well-known", FileServer::from(relative!("static/.well-known")).rank(1))
        .mount("/", FileServer::from(relative!("static")).rank(2))
}