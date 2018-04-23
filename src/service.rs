use std::io;

use fern;
use gotham;
use gotham::router::{Router, builder::*};
use gotham::pipeline::{new_pipeline, single::single_pipeline};
use gotham_middleware_postgres::PostgresMiddleware;
use gotham_rest::ResourceRouterBuilder;
use log::{self, LevelFilter};

use handlers::resources::{Bookings, Rooms};

pub struct BookingWebService {
    addr: &'static str,
    database_url: String,
}

impl BookingWebService {
    pub fn new(addr: &'static str, database_url: String) -> Self {
        BookingWebService {
            addr: addr,
            database_url: database_url,
        }
    }

    fn set_logging(&self) {
        fern::Dispatch::new()
            .level(LevelFilter::Error)
            .level_for("booking_service", LevelFilter::Debug)
            .level_for("gotham", log::LevelFilter::Info)
            .chain(io::stdout())
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}][{}]{}",
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .apply()
            .unwrap();
    }

    fn router(&self) -> Router {
        let pg_mw = PostgresMiddleware::new(self.database_url.as_str());
        let (chain, pipelines) = single_pipeline(new_pipeline().add(pg_mw).build());

        build_router(chain, pipelines, |route| {
            route.resource::<Bookings>("/bookings");
            route.resource::<Rooms>("/rooms");
        })
    }

    pub fn run(&self) {
        self.set_logging();
        gotham::start(self.addr, self.router());
    }
}
