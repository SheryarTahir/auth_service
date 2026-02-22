pub fn login(cred: models::Credentials) {
    // try to login user
    crate::database::get_user();
}

pub mod models;