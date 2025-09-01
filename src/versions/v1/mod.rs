use rocket::http::Status;

pub mod save;
pub mod extdata;
pub mod title;

#[get("/v1/status")]
pub fn v1_status() -> Status {
    Status::NoContent
}