#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;

mod ui;

fn main() {
    rocket::ignite().mount(
        "/", 
        routes![ui::index]
    ).attach(Template::fairing())
    .launch();
}
