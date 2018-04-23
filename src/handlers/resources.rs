use std::str::FromStr;

use chrono::naive::NaiveDate;
use failure;
use futures::{Future, IntoFuture};
use futures_state_stream::StateStream;
use gotham::handler::HandlerFuture;
use gotham::state::{FromState, State};
use gotham_middleware_postgres::PostgresMiddlewareData;
use gotham_rest::Resource;
use gotham_serde_json_body_parser::{create_json_response, JSONBody};
use hyper::StatusCode;

use models::Room;

pub struct Rooms;

impl Resource for Rooms {
    type Id = i32;

    fn index(state: State) -> Box<HandlerFuture> {
        let f = {
            let postgres = PostgresMiddlewareData::borrow_from(&state);

            postgres.connect(|connection| {
                debug!("connected to database");

                let f = connection
                    .prepare("SELECT id, name, bookable FROM rooms")
                    .and_then(|(select, connection)| {
                        connection
                            .query(&select, &[])
                            .map(|row| Room::new(row.get(0), row.get(1), row.get(2)))
                            .collect()
                    })
                    .and_then(|(res, _)| {
                        debug!("loaded {} rooms", res.len());
                        Ok(res)
                    })
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
                error!("Could not load rooms: {}", err);
                let res =
                    create_json_response(&state, StatusCode::InternalServerError, &"").unwrap();
                Ok((state, res))
            }
        });

        Box::new(f)
    }
}

pub struct Bookings;

#[derive(Debug, Deserialize)]
struct BookingRequestData {
    date: String,
    stays: i32,
    persons: i32,
    rooms: i32,
    firstname: String,
    lastname: String,
    telephone: String,
    email: String,
}

impl BookingRequestData {
    fn parse(self) -> Result<BookingData, failure::Error> {
        Ok(BookingData {
            date: NaiveDate::from_str(&self.date)?,
            stays: self.stays,
            persons: self.persons,
            rooms: self.rooms,
            firstname: self.firstname,
            lastname: self.lastname,
            telephone: self.telephone,
            email: self.email,
        })
    }
}

#[derive(Debug)]
struct BookingData {
    date: NaiveDate,
    stays: i32,
    persons: i32,
    rooms: i32,
    firstname: String,
    lastname: String,
    telephone: String,
    email: String,
}

impl Resource for Bookings {
    type Id = i32;

    /// curl localhost:7878/bookings -X PUT
    ///  -d '{"date": "2018-10-20", "stays": 1, "persons": 2, "rooms": 4, "firstname": "Christoph", "lastname": "Wurst", "telephone": "", "email": ""}'
    fn create(state: State) -> Box<HandlerFuture> {
        Box::new(
            state
                .json::<BookingRequestData>()
                .and_then(|(state, data)| {
                    let postgres = { (*PostgresMiddlewareData::borrow_from(&state)).clone() };

                    data.parse()
                        .into_future()
                        .map_err(|err| format_err!("parsing error: {}", err))
                        .and_then(move |booking_request| {
                            info!("received booking request: {:?}", booking_request);

                            postgres
                                .connect(|connection| {
                                    let f = connection
                                        .prepare("SELECT id, name, bookable FROM rooms")
                                        .and_then(|(select, connection)| {
                                            connection
                                                .query(&select, &[])
                                                .map(|row| {
                                                    Room::new(row.get(0), row.get(1), row.get(2))
                                                })
                                                .collect()
                                        })
                                        .and_then(|(res, _)| {
                                            debug!("loaded {} rooms", res.len());
                                            Ok(res)
                                        })
                                        .map_err(|(err, _)| err);

                                    Box::new(f)
                                })
                                .map_err(|err| format_err!("db error: {}", err))
                        })
                        .then(|_| {
                            let res =
                                create_json_response(&state, StatusCode::InternalServerError, &"")
                                    .unwrap();
                            Ok((state, res))
                        })
                }),
        )
    }
}
