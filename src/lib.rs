#![allow(dead_code, unused_variables)]

mod database;

pub mod auth_utils;

use auth_utils::login;
use database::{connect_to_database, Status};
pub fn authenticate(cred: auth_utils::models::Credentials) {
    if let Status::Connected = connect_to_database() {
        login(cred);
    }
}