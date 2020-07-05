
#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;

mod temperature;

fn main() {
    temperature::router::create_routes();
}



