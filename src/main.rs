#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate lazy_static;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use std::thread;

mod appconfig;
mod pages;

lazy_static! {
    static ref SETTINGS: appconfig::Settings = appconfig::Settings::try_load();
}

fn start_rocket() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                pages::index,
                pages::overview,
                pages::mud_runner,
                pages::snow_runner
            ],
        )
        .mount("/images", StaticFiles::from("./images"))
        .mount("/static", StaticFiles::from("./static"))
        .attach(Template::fairing())
        .launch();
}

fn start_ui() {
    let res = SETTINGS.get_resolution();
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
    let _ = thread::spawn(|| {
        start_rocket()
    });
    thread::sleep(std::time::Duration::from_secs(1));
    start_ui();

    std::process::exit(0);
}

fn start_headless() {
    start_rocket();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
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
