use crate::temperature::handlers;
use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![
        handlers::celsius,
        handlers::celsius_kelvin,
        handlers::fahrenheit
    ]
}