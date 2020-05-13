use rocket::http::Status;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Result<Template, Status> {
    Ok(Template::render("index", ()))
}

#[get("/mud-runner")]
pub fn mud_runner() -> Result<Template, Status> {
    Ok(Template::render("mudrunner", ()))
}
#[get("/snow-runner")]
pub fn snow_runner() -> Result<Template, Status> {
    Ok(Template::render("snowrunner", ()))
}