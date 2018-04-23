use futures::future::ok;
use gotham::handler::HandlerFuture;
use gotham::state::State;
use gotham_rest::Resource;
use gotham_serde_json_body_parser::create_json_response;

pub struct Rooms;

impl Resource for Rooms {
    type Id = i32;
}
