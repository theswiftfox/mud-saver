use actix_web::{web, HttpResponse, Result};
use handlebars::Handlebars;
use serde::Deserialize;

use crate::appconfig::Settings;
use crate::error::AppError;
use crate::mud_runner_saves::MudrunnerSave;
use crate::snowrunner::SnowRunnerProfile;
use crate::SETTINGS;

#[get("/check")]
pub fn check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/exit")]
pub fn exit() -> HttpResponse {
    std::process::exit(0);
}

#[get("/")]
pub async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let color = match SETTINGS.lock() {
        Ok(s) => s.get_color(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let body = match hb.render("base", &color) {
        Ok(b) => b,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    HttpResponse::Ok().body(body)
}

#[get("/overview")]
pub async fn overview(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    match hb.render("index", &()) {
        Ok(b) => HttpResponse::Ok().body(b),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/mud-runner")]
pub async fn mud_runner(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse> {
    let avail_saves = MudrunnerSave::get_available_mudrunner_saves()?;
    match hb.render("mudrunner", &avail_saves) {
        Ok(b) => Ok(HttpResponse::Ok().body(b)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[derive(Deserialize)]
pub struct MudrunnerSaveRequest {
    original_name: String,
    user_name: String,
}

#[post("/mud-runner/save")]
pub async fn store_mudrunner_save(
    params: web::Query<MudrunnerSaveRequest>,
) -> Result<HttpResponse, AppError> {
    MudrunnerSave::archive_savegame(&params.user_name, &params.original_name)?;
    Ok(HttpResponse::Ok().finish())
}

// /* *** SNOW RUNNER *** */
#[get("/snow-runner")]
pub async fn snow_runner(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse> {
    let profiles = SnowRunnerProfile::get_available_snowrunner_profiles()?;
    match hb.render("snowrunner", &profiles) {
        Ok(b) => Ok(HttpResponse::Ok().body(b)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct SnowRunnerProfileRequest {
    id: String,
}

#[get("/snow-runner/profile")]
pub async fn get_snowrunner_profile(
    hb: web::Data<Handlebars<'_>>,
    params: web::Query<SnowRunnerProfileRequest>,
) -> Result<HttpResponse, AppError> {
    let profile = SnowRunnerProfile::get_snowrunner_profile(&params.id)?;
    let saves = profile.get_archived_snowrunner_saves();
    match hb.render("snowrunner-saves", &saves) {
        Ok(b) => Ok(HttpResponse::Ok().body(b)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[get("/mud-runner/profile")]
pub async fn get_mudrunner_profile(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse, AppError> {
    match MudrunnerSave::get_archived_mudrunner_saves() {
        Ok(saves) => {
            match hb.render("mudrunner-saves", &saves) {
                Ok(b) => {
                    Ok(HttpResponse::Ok().body(b))
                }
                Err(e) => {
                    dbg!(&e);
                    Ok(HttpResponse::InternalServerError().finish())
                }
            }
        }
        Err(e) => {
            dbg!(&e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[derive(Deserialize)]
pub struct SnowRunnerProfileSaveRequest {
    id: String,
    name: String,
}

#[post("/snow-runner/profile")]
pub async fn store_snowrunner_profile(
    params: web::Query<SnowRunnerProfileSaveRequest>,
) -> Result<HttpResponse, AppError> {
    let mut profile = SnowRunnerProfile::get_snowrunner_profile(&params.id)?;
    profile.archive_savegame(&params.name)?;
    Ok(HttpResponse::Ok().finish())
}

#[delete("/snow-runner/profile?<id>&<savegame>")]
pub async fn delete_snow_runner_save(
    params: web::Query<SnowRunnerProfileSaveRequest>,
) -> Result<HttpResponse, AppError> {
    let mut profile = SnowRunnerProfile::get_snowrunner_profile(&params.id)?;
    profile.delete_archived_savegame(&params.name)?;
    Ok(HttpResponse::Ok().finish())
}

#[put("/snow-runner/profile?<id>&<savegame>")]
pub async fn restore_snow_runner_save(
    params: web::Query<SnowRunnerProfileSaveRequest>,
) -> Result<HttpResponse, AppError> {
    let mut profile = SnowRunnerProfile::get_snowrunner_profile(&params.id)?;
    profile.restore_backup(&params.name)?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/settings")]
pub fn settings() -> HttpResponse {
    match SETTINGS.lock() {
        Ok(s) => HttpResponse::Ok().json(s.clone()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/settings")]
pub async fn save_settings(settings_json: web::Json<Settings>) -> HttpResponse {
    match settings_json.store() {
        Ok(_) => {}
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match match SETTINGS.lock() {
        Ok(mut s) => s.reload(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    } {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
