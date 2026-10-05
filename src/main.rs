#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::status::Custom;

/// Messages longer than this are rejected (~1/4 of the sample messages.txt).
const MAX_LEN: usize = 200;

#[post("/notify", data = "<message>")]
fn notify(message: String) -> Result<String, Custom<String>> {
    if message.len() > MAX_LEN {
        return Err(Custom(
            Status::UnprocessableEntity,
            format!("message too long ({} > {MAX_LEN} bytes)", message.len()),
        ));
    }
    Ok(message)
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(("port", 8080));
    rocket::custom(figment).mount("/", routes![notify])
}
