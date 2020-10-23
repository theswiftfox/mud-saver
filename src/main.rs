#![cfg_attr(
    all(feature = "embed_ui", not(debug_assertions)),
    windows_subsystem = "windows"
)]

extern crate actix_files;
#[macro_use]
extern crate actix_web;
extern crate chrono;
extern crate dirs;
#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate uuid;
extern crate zip;

use chrono::{
    offset::{Local, Utc},
    DateTime,
};

use actix_web::{web, App, HttpServer};
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use listenfd::ListenFd;

use std::sync::Mutex;

mod appconfig;
mod error;
mod mud_runner_saves;
mod pages;
mod snowrunner;

lazy_static! {
    static ref SETTINGS: Mutex<appconfig::Settings> = Mutex::new(appconfig::try_load());
}

const APP_DATA_NAME: &'static str = "MudSaver";

pub fn get_app_data_dir() -> std::path::PathBuf {
    let path = dirs::data_dir();
    if path.is_some() {
        let mut p = path.unwrap();
        p.push(APP_DATA_NAME);
        if !p.exists() {
            std::fs::create_dir(&p).unwrap();
        }
        p
    } else {
        std::env::current_dir().unwrap()
    }
}

#[actix_rt::main]
async fn start_rocket() {
    let mut handlebars = handlebars::Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./templates")
        .unwrap();
    handlebars.register_helper(
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
                    if let Ok(date) = serde_json::from_value::<DateTime<Utc>>(date_js.clone()) {
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
    let hb_ref = web::Data::new(handlebars);
    let mut server = HttpServer::new(move || {
        App::new()
            // .wrap(middleware::Logger::default())
            .app_data(hb_ref.clone())
            .service(pages::exit)
            .service(pages::index)
            .service(pages::check)
            .service(pages::overview)
            .service(pages::settings)
            .service(pages::mud_runner)
            .service(pages::snow_runner)
            .service(pages::settings)
            .service(pages::save_settings)
            .service(pages::store_snowrunner_profile)
            .service(pages::get_snowrunner_profile)
            .service(pages::delete_snow_runner_save)
            .service(pages::restore_snow_runner_save)
            .service(pages::store_mudrunner_save)
            .service(pages::restore_mud_runner_save)
            .service(pages::get_mudrunner_profile)
            .service(pages::update_snow_runner_profile_alias)
            .service(actix_files::Files::new("/static", "./static"))
            .service(actix_files::Files::new("/images", "./images"))
    });
    server = if let Some(l) = ListenFd::from_env().take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8000").unwrap()
    };
    server.run()
    .await
    .unwrap();
}

#[cfg(feature = "embed_ui")]
mod webview;

#[cfg(feature = "embed_ui")]
#[actix_rt::main]
async fn main() {
    std::process::exit(match webview::main_ui().await {
        Ok(_) => 0,
        Err(_) => {
            eprintln!("Error while running application.");
            1
        }
    });
}

#[cfg(not(feature = "embed_ui"))]
fn main() {
    get_app_data_dir();
    start_rocket();
}
