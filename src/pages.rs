use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use crate::appconfig::Settings;
use crate::snowrunner::SnowRunnerSave;
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
    let profiles = match SnowRunnerSave::get_available_snowrunner_saves() {
        Ok(p) => p,
        Err(e) => return Err(json!(e)),
    };
    Ok(Template::render("snowrunner", profiles))
}

#[post("/snow-runner?<profile>&<name>")]
pub fn store_snow_runner_profile(
    profile: Option<String>,
    name: Option<String>,
) -> Result<(), JsonValue> {
    if profile.is_none() {
        return Err(json!((Status::BadRequest, "Missing profile parameter")));
    }
    if name.is_none() {
        return Err(json!((Status::BadRequest, "Missing name parameter")));
    }
    let mut profile = match SnowRunnerSave::get_snowrunner_profile(&profile.unwrap()) {
        Ok(p) => p,
        Err(e) => {
            return Err(json!((Status::InternalServerError, e.to_string())));
        }
    };
    match profile.archive_savegame(&name.unwrap()) {
        Ok(_) => Ok(()),
        Err(e) => Err(json!((Status::InternalServerError, e.to_string()))),
    }
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
