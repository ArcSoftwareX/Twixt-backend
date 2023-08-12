use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;

use crate::model::{auth::JwtMiddleware, post::Post, state::AppState};

// #[post("/file-upload")]
// async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
//     let mut files = Vec::new();

//     while let Some(mut field) = payload.try_next().await? {
//         let content_type = field.content_type();

//         if content_type.is_none() {
//             return Ok(
//                 HttpResponse::BadRequest().json(json!({ "message": "No mime type provided" }))
//             );
//         }

//         let content_type = content_type.unwrap();
//         let type_ = content_type.type_();

//         if type_ != "image" && type_ != "video" {
//             return Ok(HttpResponse::BadRequest().json(json!({
//                 "message": format!("Invalid mime type: {}", type_)
//             })));
//         }

//         let filename = uuid::Uuid::new_v4().to_string();
//         let filepath = format!(
//             "./storage/{}/{filename}.{}",
//             type_,
//             field
//                 .content_disposition()
//                 .get_filename()
//                 .unwrap()
//                 .split('.')
//                 .last()
//                 .unwrap()
//         );

//         let mut f = web::block(|| std::fs::File::create(filepath)).await??;

//         while let Some(chunk) = field.try_next().await? {
//             f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
//         }

//         files.push(format!(
//             "{}/{filename}",
//             field.content_type().unwrap().type_()
//         ))
//     }

//     Ok(HttpResponse::Ok().json(json!({ "files": files })))
// }

#[derive(MultipartForm)]
#[multipart(duplicate_field = "deny")]
struct PostCreateInput {
    content: Text<String>,

    #[multipart(limit = "100 MiB")]
    media: Vec<TempFile>,
}

#[post("/create")]
async fn create_post(
    MultipartForm(form): MultipartForm<PostCreateInput>,
    data: web::Data<AppState>,
    jwt: JwtMiddleware,
) -> impl Responder {
    let mut files = Vec::new();

    for file in form.media {
        let ct = file.content_type.unwrap();
        let ct = ct.type_();
        if ct != "image" && ct != "video" {
            return HttpResponse::BadRequest().json(json!({
                "message": format!("Unsupported content type: {}", ct)
            }));
        }
        let path = format!(
            "./storage/{}/{}.{}",
            ct,
            uuid::Uuid::new_v4(),
            file.file_name.unwrap().split('.').last().unwrap()
        );

        file.file.persist(&path).unwrap();
        files.push(path.split_at(10).1.to_string());
    }

    let query_res = sqlx::query_as!(
        Post,
        "INSERT INTO posts (content, media_links, author_id) VALUES ($1, $2, $3) RETURNING *",
        form.content.to_string(),
        &files,
        jwt.user_id
    )
    .fetch_one(&data.db_pool)
    .await;

    if query_res.is_err() {
        return HttpResponse::InternalServerError()
            .json(json!({ "message": "An internal error occured" }));
    }

    let query_res = query_res.unwrap();

    HttpResponse::Ok().json(query_res)
}
