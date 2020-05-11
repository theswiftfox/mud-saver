use rocket::http::Status;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Result<Template, Status> {
    Ok(Template::render("index", ()))
}