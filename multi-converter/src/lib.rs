
pub fn celsius_to_faherenheit(c: f32) -> f32 {
    (c * 1.8) + 32.0
}
pub fn celsius_to_kelvin(c: f32) -> f32 {
    c + 273.15
}
pub fn faherenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 0.55
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
