extern crate booking_service;

use booking_service::BookingWebService;

fn main() {
    let addr = "127.0.0.1:7878";
    let web_service = BookingWebService::new(addr);
    println!("Starting booking service on http://{}", addr);
    web_service.run();
}
