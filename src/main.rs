use std::env;

use actix_web::{
    middleware, web, App, HttpServer,
};
use dotenv::dotenv;
use listenfd::ListenFd;

mod todo;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("/todo").configure(todo::routes_config))
    });

    env_logger::init();

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Set host in .env");
            let port = env::var("PORT").expect("Set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await
}
