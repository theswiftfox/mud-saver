#![cfg_attr(feature = "embed_ui", windows_subsystem = "windows")]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate lazy_static;

use chrono::{
    offset::{Local, Utc},
    DateTime,
};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[cfg(feature = "embed_ui")]
use std::thread;

use std::sync::Mutex;

mod appconfig;
mod pages;

lazy_static! {
    static ref SETTINGS: Mutex<appconfig::Settings> = Mutex::new(appconfig::try_load());
}

fn start_rocket() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                pages::index,
                pages::overview,
                pages::mud_runner,
                pages::snow_runner,
                pages::settings,
                pages::save_settings,
            ],
        )
        .mount("/images", StaticFiles::from("./images"))
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .launch();
}

#[cfg(feature = "embed_ui")]
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

#[cfg(feature = "embed_ui")]
fn main() {
    let _ = thread::spawn(|| start_rocket());
    thread::sleep(std::time::Duration::from_secs(1));
    start_ui();

    std::process::exit(0);
}


#[cfg(not(feature = "embed_ui"))]
fn main() {
    start_rocket();
}
