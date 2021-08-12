use actix_web::{
    middleware, web, App, HttpServer,
};

use todo_api::routes::todo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("/todo").configure(todo::todos))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
