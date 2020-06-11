use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

use crate::appconfig::Settings;
use crate::error::AppError;
use crate::snowrunner::SnowRunnerProfile;
use crate::SETTINGS;

#[get("/check")]
pub fn check() -> Result<(), AppError> {
    Ok(())
}

#[post("/exit")]
pub fn exit() -> Result<(), AppError> {
    std::process::exit(0);
}

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

/* *** SNOW RUNNER *** */
#[get("/snow-runner")]
pub fn snow_runner() -> Result<Template, JsonValue> {
    let profiles = match SnowRunnerProfile::get_available_snowrunner_profiles() {
        Ok(p) => p,
        Err(e) => return Err(json!(e)),
    };
    Ok(Template::render("snowrunner", profiles))
}

#[get("/snow-runner/profile?<id>")]
pub fn get_snowrunner_profile(id: Option<String>) -> Result<Template, AppError> {
    if id.is_none() {
        return Err(AppError::MissingParameter(String::from("id")));
    }
    let profile = SnowRunnerProfile::get_snowrunner_profile(&id.unwrap())?;
    let saves = profile.get_archived_snowrunner_saves();
    Ok(Template::render("snowrunner-saves", saves))
}

#[post("/snow-runner/profile?<id>&<name>")]
pub fn store_snow_runner_profile(id: Option<String>, name: Option<String>) -> Result<(), AppError> {
    if id.is_none() {
        return Err(AppError::MissingParameter(String::from("id")));
    }
    if name.is_none() {
        return Err(AppError::MissingParameter(String::from("name")));
    }
    let mut profile = SnowRunnerProfile::get_snowrunner_profile(&id.unwrap())?;
    profile.archive_savegame(&name.unwrap())
}
#[delete("/snow-runner/profile?<id>&<savegame>")]
pub fn delete_snow_runner_save(
    id: Option<String>,
    savegame: Option<String>,
) -> Result<(), AppError> {
    if id.is_none() {
        return Err(AppError::MissingParameter(String::from("id")));
    }
    if savegame.is_none() {
        return Err(AppError::MissingParameter(String::from("savegame")));
    }
    let mut profile = SnowRunnerProfile::get_snowrunner_profile(&id.unwrap())?;
    profile.delete_archived_savegame(&savegame.unwrap())
}

#[put("/snow-runner/profile?<id>&<savegame>")]
pub fn restore_snow_runner_save(
    id: Option<String>,
    savegame: Option<String>,
) -> Result<(), AppError> {
    if id.is_none() {
        return Err(AppError::MissingParameter(String::from("id")));
    }
    if savegame.is_none() {
        return Err(AppError::MissingParameter(String::from("savegame")));
    }
    let mut profile = SnowRunnerProfile::get_snowrunner_profile(&id.unwrap())?;
    profile.restore_backup(&savegame.unwrap())
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
