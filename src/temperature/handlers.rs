use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateResult {
    from: &'static str,
    to: &'static str,
    value: f32,
    result: f32,
}

#[get("/celsius/<c>")]
pub fn celsius(c: f32) -> Template {
    Template::render(
        "result",
        &TemplateResult {
            from: "celsius",
            to: "faherenheit",
            value: c,
            result: celsius_to_faherenheit(c),
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
            result: celsius_to_kelvin(c),
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
            result: faherenheit_to_celsius(f),
        },
    )
}

fn celsius_to_faherenheit(c: f32) -> f32 {
    (c * 1.8) + 32.0
}
fn celsius_to_kelvin(c: f32) -> f32 {
    c + 273.15
}
fn faherenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 0.55
}
