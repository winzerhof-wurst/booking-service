use futures::future::{ok, Future};
use futures_state_stream::StateStream;
use gotham::handler::HandlerFuture;
use gotham::state::{FromState, State};
use gotham_middleware_postgres::PostgresMiddlewareData;
use gotham_rest::Resource;
use gotham_serde_json_body_parser::create_json_response;
use hyper::StatusCode;
use tokio_core::reactor::Handle;

use models::Room;

pub struct Rooms;

impl Resource for Rooms {
    type Id = i32;

    fn index(state: State) -> Box<HandlerFuture> {
        let f = {
            let handle = Handle::borrow_from(&state);
            let postgres = PostgresMiddlewareData::borrow_from(&state);

            postgres.connect(handle, |connection| {
                debug!("connected to database");

                let f = connection
                    .prepare("SELECT id, name, bookable FROM rooms")
                    .and_then(|(select, connection)| {
                        connection.query(&select, &[]).map(|row| {
                            Room::new(row.get(0), row.get(1), row.get(2))
                        })
                        .collect()
                    })
                    .and_then(|(res, _)| Ok(res))
                    .map_err(|(err, _)| err);

                Box::new(f)
            })
        };

        let f = f.then(move |res| match res {
            Ok(rooms) => {
                let res = create_json_response(&state, StatusCode::Ok, &rooms).unwrap();
                Ok((state, res))
            }
            Err(err) => {
                let res =
                    create_json_response(&state, StatusCode::InternalServerError, &0).unwrap();
                Ok((state, res))
            }
        });

        Box::new(f)
    }
}
