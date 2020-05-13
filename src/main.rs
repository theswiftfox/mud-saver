#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use std::thread;

mod ui;

fn start_rocket() {
    rocket::ignite()
    .mount(
        "/", 
        routes![
            ui::index,
            ui::mud_runner,
            ui::snow_runner
        ]
    )
    .mount("/images", StaticFiles::from("./images"))
    .attach(Template::fairing())
    .launch();
}

fn start_ui() {
    web_view::builder()
		.title("MudSaver")
		.content(web_view::Content::Url("http://localhost:8000"))
		.size(800, 600)
		.resizable(true)
		.debug(true)
		.user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}

fn with_ui() {
    let _ = thread::spawn(|| {
        start_rocket()
    });
    thread::sleep(std::time::Duration::from_secs(1));
    start_ui();

    std::process::exit(0);
}

fn headless() {
    start_rocket();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if &args[1] == "--with-ui" {
            with_ui()
        } else {
            println!("argument not recognized")
        }
    } else {
        headless()
    }
}
