use actix_web::{web, Error, HttpResponse};

use crate::todo::models::Todo;


async fn get_todo(web::Path(id): web::Path<u32>) -> Result<HttpResponse, Error> {
    println!("get_todo");
    let id_option: Option<u32> = Some(id);
    Ok(
        HttpResponse::Ok().json(Todo {
        id: id_option,
        content: String::from("やること"),
        done: false,
    }))
}

async fn get_todo_list() -> Result<HttpResponse, Error> {
    println!("get_todo_list");
    let todo_list = [
        Todo {
            id: Some(1),
            content: String::from("１こめ"),
            done: false,
        },
        Todo {
            id: Some(2),
            content: String::from("２こめ"),
            done: false,
        },
        Todo {
            id: Some(3),
            content: String::from("３こめ"),
            done: false,
        }
    ];
    Ok(
        HttpResponse::Ok().json(
            todo_list
        ))
}

async fn post_todo(todo: web::Json<Todo>) -> Result<HttpResponse, Error> {
    println!("post_todo");
    println!("{:?}", todo);
    Ok(
        HttpResponse::Ok().body("ok")
    )
}

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/item/{id}", web::get().to(get_todo))
        .route("/list", web::get().to(get_todo_list))
        .route("/create", web::post().to(post_todo));
}