use rocket::{get, launch, post, routes};
// use tokio::runtime::Builder;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct User {
    id: usize,
    username: String,
    password: Option<String>,
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>", rank = 2)]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/hello/<count>")]
fn counter(count: u32) -> String {
    format!("Counter {}", count)
}

#[get("/hello")]
fn hi() -> &'static str {
    "Hi!"
}

#[get("/")]
fn hidden_index() -> &'static str {
    "Hidden: Hello, world!"
}

#[post("/user", format = "application/json", data = "<user>")]
fn new_user(user: Json<User>) -> (Status, Result<Json<User>, ()>) {
    let nuser = User {
        id: 10,
        username: String::from("iot"),
        password: None,
    };
    if user.id == 0 {
        (Status::Ok, Ok(Json::from(nuser)))
    } else {
        (Status::Unauthorized, Err(()))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hi, hello, counter, new_user])
        .mount("/hidden", routes![hidden_index])
}

// #[rocket::main]
// async fn main() {
//     let server = rocket::build()
//         .mount("/hello", routes![index]);
//     server.launch().await;
// }

// fn main() {
//     let rt = Builder::new_current_thread().enable_all().build().unwrap();
//     rt.block_on(async {
//         let server = rocket::build().mount("/hello", routes![index]);
//         server.launch().await;
//     });
// }
