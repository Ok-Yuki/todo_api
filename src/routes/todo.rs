use actix_web::web;

use crate::controllers::{todo};

pub fn todos(cfg: &mut web::ServiceConfig) {
    cfg.route("/item/{id}", web::get().to(todo::get_todo))
        .route("/list", web::get().to(todo::get_todo_list))
        .route("/create", web::post().to(todo::post_todo));
}
