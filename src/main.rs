#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod temperature;

fn main() {
    let rocket = rocket::ignite()
        .mount("/public", StaticFiles::from("public"))
        .mount("/", routes![index])
        .mount("/temperature", temperature::router::get_routes())
        .attach(Template::fairing());

    rocket.launch();
}

#[get("/")]
fn index() -> Template {
    Template::render("index", ())
}