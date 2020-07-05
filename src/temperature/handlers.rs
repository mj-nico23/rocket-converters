#[get("/celsius/<c>")]
pub fn celsius(c: f32) -> String {
    format!("{} celsius are {} fahrenheit", c, celsius_to_faherenheit(c))
}

#[get("/celsius-kelvin/<c>")]
pub fn celsius_kelvin(c: f32) -> String {
    format!("{} celisus are {} kelvin", c, celsius_to_kelvin(c))
}

#[get("/fahrenheit/<f>")]
pub fn fahrenheit(f: f32) -> String {
    format!("{} fahrenheit are {} celsius", f, faherenheit_to_celsius(f))
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
