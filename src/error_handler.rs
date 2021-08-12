use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
enum CustomError {}

impl ResponseError for CustomError {}