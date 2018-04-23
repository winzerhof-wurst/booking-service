use std::io;

use fern;
use gotham;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham_rest::ResourceRouterBuilder;
use log::{self, LevelFilter};

use handlers::resources::Rooms;

pub struct BookingWebService {
    addr: &'static str,
}

impl BookingWebService {
    pub fn new(addr: &'static str) -> Self {
        BookingWebService {
            addr: addr
        }
    }

    fn set_logging(&self) {
        fern::Dispatch::new()
            .level(LevelFilter::Error)
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
        let (chain, pipelines) = single_pipeline(new_pipeline().build());

        build_router(chain, pipelines, |route| {
            route.resource::<Rooms>("/rooms");
        })
    }

    pub fn run(&self) {
        self.set_logging();
        gotham::start(self.addr, self.router());
    }
}
