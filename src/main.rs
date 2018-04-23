extern crate booking_service;

use std::env;

use booking_service::BookingWebService;

static DATABASE_URL: &str = "postgresql://postgres:mysecretpassword@localhost:5432";

fn main() {
    let addr = "127.0.0.1:7878";
    let database_url = env::var("DATABASE_URL").unwrap_or(DATABASE_URL.to_owned());

    println!("Using {} as database URL", database_url);
    println!("Starting booking service on http://{}", addr);
    let web_service = BookingWebService::new(addr, database_url);
    web_service.run();
}
