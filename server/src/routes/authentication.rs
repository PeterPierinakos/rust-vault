use actix_web::{Responder};
use std::io::Error;

pub fn signup() -> Result<impl Responder, Error> {
    Ok("Success")
}
