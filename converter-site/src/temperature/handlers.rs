use rocket_contrib::templates::Template;
use rocket::response::Redirect;

use multi_converter;

#[derive(Serialize)]
struct TemplateResult {
    from: &'static str,
    to: &'static str,
    value: f32,
    result: f32,
}

#[get("/")]
pub fn index() -> Template {
    Template::render("temperature/index", ())
}

#[get("/celsius/<c>")]
pub fn celsius(c: f32) -> Template {
    Template::render(
        "temperature/index",
        &TemplateResult {
            from: "celsius",
            to: "faherenheit",
            value: c,
            result: multi_converter::celsius_to_faherenheit(c),
        },
    )
}

#[get("/celsius-kelvin/<c>")]
pub fn celsius_kelvin(c: f32) -> Template {
    Template::render(
        "result",
        &TemplateResult {
            from: "celsius",
            to: "kelvin",
            value: c,
            result: multi_converter::celsius_to_kelvin(c),
        },
    )
}

#[get("/fahrenheit/<f>")]
pub fn fahrenheit(f: f32) -> Template {
    Template::render(
        "result",
        &TemplateResult {
            from: "faherenheit",
            to: "celsius",
            value: f,
            result: multi_converter::faherenheit_to_celsius(f),
        },
    )
}

#[get("/convert?<value>")]
pub fn convert(value: f32) -> Redirect {
    Redirect::to(format!("/temperature/celsius/{}", value))
}