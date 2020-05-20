use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use crate::appconfig::Settings;
use crate::snowrunner::SnowRunnerProfile;
use crate::SETTINGS;

#[get("/")]
pub fn index() -> Result<Template, Status> {
    let color = match SETTINGS.lock() {
        Ok(s) => s.get_color(),
        Err(_) => return Err(Status::InternalServerError),
    };
    Ok(Template::render("base", color))
}

#[get("/overview")]
pub fn overview() -> Result<Template, Status> {
    Ok(Template::render("index", ()))
}

#[get("/mud-runner")]
pub fn mud_runner() -> Result<Template, Status> {
    Ok(Template::render("mudrunner", ()))
}
#[get("/snow-runner")]
pub fn snow_runner() -> Result<Template, JsonValue> {
    let profiles = match SnowRunnerProfile::load_profiles_from_disk() {
        Ok(p) => p,
        Err(e) => return Err(json!(e)),
    };
    dbg!(&profiles);
    Ok(Template::render("snowrunner", profiles))
}

#[get("/settings")]
pub fn settings() -> Result<Json<Settings>, Status> {
    match SETTINGS.lock() {
        Ok(s) => Ok(Json(s.clone())),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/settings", format = "json", data = "<settings>")]
pub fn save_settings(settings: Json<Settings>) -> Result<(), Status> {
    match settings.store() {
        Ok(_) => {}
        Err(_) => return Err(Status::InternalServerError),
    };

    match match SETTINGS.lock() {
        Ok(mut s) => s.reload(),
        Err(_) => return Err(Status::InternalServerError),
    } {
        Ok(_) => Ok(()),
        Err(_) => Err(Status::InternalServerError),
    }
}
