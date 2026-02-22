   
pub enum Status {
    Connected,
    Interupted,
}

pub fn connect_to_database() -> Status {
    // connect to db.. -> logic here
    Status::Connected
}

pub fn get_user() {
    // fetech the user from db and return
}