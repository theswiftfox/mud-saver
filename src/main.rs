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

use chrono::{offset::{Local, Utc}, DateTime};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::{
    handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError},
    Template,
};

use std::sync::Mutex;
use std::thread;

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

fn start_ui() {
    let res = (1000, 600);
    web_view::builder()
        .title("MudSaver")
        .content(web_view::Content::Url("http://localhost:8000"))
        .size(res.0 as i32, res.1 as i32)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}

fn start_with_ui() {
    let _ = thread::spawn(|| start_rocket());
    thread::sleep(std::time::Duration::from_secs(1));
    start_ui();

    std::process::exit(0);
}

fn start_headless() {
    start_rocket();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let data_dir = get_app_data_dir().unwrap();
    if !data_dir.exists() {
        match std::fs::create_dir(&data_dir) {
            Ok(_) => (),
            Err(e) => {
                dbg!(&data_dir, &e);
                panic!(error::AppError::FileWriteError(String::from("Unable to create data directory for backups.")));
            }
        }
    }
    if args.len() > 1 {
        if &args[1] == "--with-ui" {
            start_with_ui()
        } else {
            println!("argument not recognized")
        }
    } else {
        start_headless()
    }
}
