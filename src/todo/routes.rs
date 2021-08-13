use actix_web::{web, Error, HttpResponse};

use crate::todo::models::{Todo, Todos};


async fn get_item(web::Path(id): web::Path<i32>) -> Result<HttpResponse, Error> {
    let todo = Todos::find(id)?;
    Ok(
        HttpResponse::Ok().json(todo)
    )
}

async fn get_list() -> Result<HttpResponse, Error> {
    let todos = Todos::find_all()?;
    Ok(
        HttpResponse::Ok().json(todos)
    )
}

async fn create_item(web::Json(todo): web::Json<Todo>) -> Result<HttpResponse, Error> {
    let todo = Todos::create(todo)?;
    Ok(
        HttpResponse::Ok().json(todo)
    )
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/item/{id}", web::get().to(get_item))
        .route("/list", web::get().to(get_list))
        .route("/create", web::post().to(create_item));
}