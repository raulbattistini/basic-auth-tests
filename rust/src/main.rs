// use std::env;
// // use core::num::flt2dec::decode;
// // use std::error::Error;
//
// use actix_web::{get, post, web, App, HttpResponse, HttpRequest, HttpServer, Responder};
// use actix_web_httpauth::extractors::basic::{self, BasicAuth};
//
// const USERNAME: String = env::var("USERNAME").unwrap();
// const PASSWORD: String = env::var("PASSWORD").unwrap();
//
// async fn index(auth: BasicAuth) -> String {
//     format!("Hello, {}!", auth.user_id())
// }
//
// const USERNAME: String = env::var("USERNAME").unwrap();
// const PASSWORD: String = env::var("PASSWORD").unwrap();
// struct Auth {
//    username: String,
//     password: String
// }
//
// static ENV_AUTH: Auth = Auth{
//     username: USERNAME,
//     password: PASSWORD
// };
//
// // async fn validate(provided: BasicAuth, expected: Auth) -> bool {
// //     let provided_id = provided.user_id();
// //     let provided_password = provided.password();
// //
// //     let expected_id = expected.username;
// //     let expected_password = expected.password;
// //
// //     if provided_id != expected_id || provided_password != expected_password {
// //         return false
// //     }
// //     return true
// // }
//
// #[post("/webhook-sample")]
// async fn attempt(req: HttpRequest) -> impl Responder {
//     // Accessing the headers
//     let headers = req.headers();
//     let user_agent = headers.get("").unwrap().to_str().unwrap_or("Unknown User-Agent");
//     let mut username = "Unknown";
//     let mut password = "";
//
//     if let Some(auth_header) = req.headers().get("Authorization") {
//         if let Ok(auth_str) = auth_header.to_str() {
//             if auth_str.starts_with("Basic ") {
//                 // Extract the encoded credentials
//                 let encoded_credentials = auth_str.trim_start_matches("Basic ");
//
//                 // Decode the credentials
//                 if let Ok(decoded_bytes) = decode(encoded_credentials) {
//                     if let Ok(decoded_str) = std::str::from_utf8(&decoded_bytes) {
//                         // Split the decoded credentials into username and password
//                         let credentials: Vec<&str> = decoded_str.splitn(2, ':').collect();
//                         if credentials.len() == 2 {
//                             username = credentials[0];
//                             password = credentials[1];
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     let expected_pass = ENV_AUTH.password;
//     let expected_user = ENV_AUTH.username;
//     if username != ENV_AUTH.username || password != ENV_AUTH.password {
//         println!("it was provided either {:?} instead of {:?} or {:?} instead of ${:?}", username, expected_user, password, expected_pass);
//     }
//  HttpResponse::Ok().body(format!("Hello, {}! Your password is: {}", username, password));
// }
//
// // #[post("/echo")]
// // async fn echo(req_body: String) -> impl Responder {
// //
// //     HttpResponse::Ok().body(req_body)
// // }
// //
// // async fn manual_hello() -> impl Responder {
// //     HttpResponse::Ok().body("Hey there!")
// // }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             // .service(echo)
//             // .route("/hey", web::get().to(manual_hello))
//             .route("/webhook-sample", web::post().to(attempt|| HttpResponse::MethodNotAllowed()));
//                     //
//
//
//     })
//     .bind(("127.0.0.1", 6969))?
//     .run()
//     .await
// }
//

use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest};
use base64;
use std::str;
use std::env;
use dotenv::dotenv;


struct Auth {
   username: String,
    password: String
}

async fn index(req: HttpRequest) -> HttpResponse {
let username_env  = match env::var("USERNAME"){
    Ok(v) => v.to_owned(),
    Err(e) => panic!("$USERNAME unset: {}", e)
};
let password_env: String = env::var("PASSWORD").unwrap();
    let env_auth: Auth = Auth{
        username: username_env,
        password: password_env
    };
    let headers = req.headers();
    let mut response_message = String::from("Credentials not provided");
    let mut username: String = String::new();
    let mut password: String= String::new(); 

    if let Some(auth_header_value) = headers.get("Authorization") {
        if let Ok(auth_header_str) = auth_header_value.to_str() {
            if auth_header_str.starts_with("Basic ") {
                let base64_encoded = auth_header_str.trim_start_matches("Basic ");
                match base64::decode(base64_encoded) {
                    Ok(decoded_bytes) => match str::from_utf8(&decoded_bytes) {
                        Ok(decoded_str) => {
                            let credentials: Vec<&str> = decoded_str.splitn(2, ':').collect();
                            if credentials.len() == 2 {
                                response_message = format!("Username: {}, Password: {}", credentials[0], credentials[1]);
                                let bind = credentials[0];
                                let bind_two = credentials[1];
                                username = bind.to_owned();
                                password = bind_two.to_owned();
                            }

                        },
                        Err(_) => response_message = String::from("Error decoding credentials"),
                    },
                    Err(_) => response_message = String::from("Error decoding base64 credentials"),
                }
            }
        }
    }
    let expected_pass = env_auth.password;
    let expected_user = env_auth.username;
    if username != expected_user || password != expected_pass{ 
        println!("it was provided either {:?} instead of {:?} or {:?} instead of ${:?}", username, expected_user, password, expected_pass);
    }


    HttpResponse::Ok().body(response_message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .route("/webhook-sample", web::get().to(index))
    })
    .bind("127.0.0.1:6969")?
    .run()
    .await
}