#[derive(Serialize)]
pub struct Room {
    id: i32,
    name: String,
    bookable: bool,
}

impl Room {
    pub fn new(id: i32, name: String, bookable: bool) -> Self {
        Room {
            id: id,
            name: name,
            bookable: bookable,
        }
    }
}
