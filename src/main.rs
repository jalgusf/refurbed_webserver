#[macro_use]
extern crate rocket;

use std::time::Duration;

use rocket::State;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::Deserialize;

/// Messages longer than this are rejected (~1/4 of the sample messages.txt).
const MAX_LEN: usize = 200;

/// Set via `ROCKET_DELAY_MS` or `delay_ms` in Rocket.toml.
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    /// Artificial delay before every response, to simulate a slow service.
    delay_ms: u64,
}

#[post("/notify", data = "<message>")]
async fn notify(message: String, config: &State<AppConfig>) -> Result<String, Custom<String>> {
    rocket::tokio::time::sleep(Duration::from_millis(config.delay_ms)).await;
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
    // Defaults only; ROCKET_PORT / ROCKET_DELAY_MS or Rocket.toml take precedence.
    let figment = rocket::Config::figment()
        .join(("port", 8080))
        .join(("delay_ms", 1000));
    rocket::custom(figment)
        .mount("/", routes![notify])
        .attach(AdHoc::config::<AppConfig>())
}
