use rocket;
use crate::temperature::handlers;

pub fn create_routes(){
    rocket::ignite()
        .mount("/temperature", routes![
                handlers::celsius,
                handlers::celsius_kelvin,
                handlers::fahrenheit]
        )
        .launch();
}

