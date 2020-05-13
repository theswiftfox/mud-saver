#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

mod ui;

fn main() {
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
