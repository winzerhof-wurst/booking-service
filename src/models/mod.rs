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

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_bookable(&self) -> bool {
        self.bookable
    }
}
