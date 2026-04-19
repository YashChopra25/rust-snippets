pub enum Status {
    Connected,
    INTRUPRUPTED,
}

pub fn connect_to_database() -> Status {
    Status::Connected
}
