use axum::{
    extract:: Request,
    response::Response,
    middleware::{Next}
};
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use std::io::ErrorKind;
use chrono::prelude::*;

pub async fn request_logger(req: Request, next: Next) -> Response {

    let txt_file = OpenOptions::new().append(true).open("log.txt");
    let local: DateTime<Local> = Local::now();
    let message = format!("{} '{}' at {}\n", req.method(), req.uri(), local);
    match txt_file {
        Ok(mut txt_file) => {
            let _ = txt_file.write(message.as_bytes());
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                match File::create("log.txt") {
                    Ok(mut file) => {
                        let _ = file.write(message.as_bytes());
                    }
                    Err(e) => println!("Error: {}", e)
                }
                println!("Error: {}",e);
            },
            _ => {

            }
            
        }
    }

    let res = next.run(req).await;
    res
}