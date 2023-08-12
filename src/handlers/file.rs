use std::path::Path;

use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/file/{content_type}/{id}")]
async fn file_handler(path: web::Path<(String, String)>) -> impl Responder {
    let ct = path.0.clone();
    let id = path.1.clone();
    let pathname = format!("./storage/{}", path.0);
    let ext = web::block(move || {
        for entry in std::fs::read_dir(pathname).unwrap() {
            let entry = entry.unwrap().path();
            let file = Path::new(&entry);
            println!("{:?}", file.file_name());
            if file.is_file()
                && file
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split('.')
                    .next()
                    == Some(&path.1)
            {
                return Some(file.extension().unwrap().to_string_lossy().to_string());
            }
        }

        None
    })
    .await
    .unwrap();

    println!("{ct}/{:?}", ext);

    if ext.is_none() {
        return HttpResponse::NotFound().json(json!({ "message": "Resource not found" }));
    }

    let ext = ext.unwrap();

    let pathname = format!("./storage/{}/{id}.{ext}", ct);

    let content = web::block(move || (std::fs::read(pathname), ext))
        .await
        .unwrap();

    if content.0.is_err() {
        return HttpResponse::InternalServerError()
            .json(json!({ "message": "An internal error occured" }));
    }

    let file_content = content.0.unwrap();

    HttpResponse::Ok()
        .content_type(format!("{}/{}", ct, content.1))
        .body(file_content)
}
