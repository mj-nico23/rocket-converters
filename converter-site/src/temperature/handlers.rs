use rocket_contrib::templates::Template;
use rocket::response::Redirect;

use multi_converter::{Unit, Temperature};

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
            result: Temperature::new(c, Unit::Celsius).convert(Unit::Faherenheit),
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
            result: Temperature::new(c, Unit::Celsius).convert(Unit::Kelvin),
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
            result: Temperature::new(f, Unit::Faherenheit).convert(Unit::Celsius),
        },
    )
}

#[get("/convert?<value>")]
pub fn convert(value: f32) -> Redirect {
    Redirect::to(format!("/temperature/celsius/{}", value))
}