#![cfg_attr(
    all(feature = "embed_ui", not(debug_assertions)),
    windows_subsystem = "windows"
)]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate dirs;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate uuid;
extern crate zip;

use chrono::{
    offset::{Local, Utc},
    DateTime,
};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{
    handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError},
    Template,
};

use std::sync::Mutex;

mod appconfig;
mod error;
mod pages;
mod snowrunner;

lazy_static! {
    static ref SETTINGS: Mutex<appconfig::Settings> = Mutex::new(appconfig::try_load());
}

const APP_DATA_NAME: &'static str = "MudSaver";

pub fn get_app_data_dir() -> std::io::Result<std::path::PathBuf> {
    let path = dirs::data_dir();
    if path.is_some() {
        let mut p = path.unwrap();
        p.push(APP_DATA_NAME);
        Ok(p)
    } else {
        std::env::current_dir()
    }
}

fn start_rocket() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                pages::exit,
                pages::index,
                pages::check,
                pages::overview,
                pages::mud_runner,
                pages::snow_runner,
                pages::settings,
                pages::save_settings,
                pages::store_snow_runner_profile,
                pages::get_snowrunner_profile,
                pages::delete_snow_runner_save,
                pages::restore_snow_runner_save,
            ],
        )
        .mount("/images", StaticFiles::from("./images"))
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper(
                "date-time",
                Box::new(
                    |h: &Helper,
                     _: &Handlebars,
                     _: &Context,
                     _: &mut RenderContext,
                     out: &mut dyn Output|
                     -> HelperResult {
                        if let Some(date_val) = h.hash().get("date") {
                            let date_js = date_val.value();
                            if let Ok(date) =
                                serde_json::from_value::<DateTime<Utc>>(date_js.clone())
                            {
                                let local = DateTime::<Local>::from(date);
                                let local_date = local.naive_local();
                                let date_str = format!(
                                    "{} - {}",
                                    local_date.date(),
                                    local_date.time().format("%H:%M:%S")
                                );
                                out.write(&date_str)?;
                                return Ok(());
                            }
                        }
                        Err(RenderError::new("Error parsing date"))
                    },
                ),
            );
        }))
        .launch();
}

#[cfg(feature = "embed_ui")]
mod webview;

#[cfg(feature = "embed_ui")]
fn main() {
    std::process::exit(match webview::main_ui() {
        Ok(_) => 0,
        Err(_) => {
            eprintln!("Error while running application.");
            1
        }
    });
}

#[cfg(not(feature = "embed_ui"))]
fn main() {
    start_rocket();
}
