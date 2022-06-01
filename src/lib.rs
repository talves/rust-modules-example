#![allow(dead_code, unused_variables)]
mod auth_utils;
mod database;

pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = crate::database::connect_to_database() {
        crate::auth_utils::login(creds);
    }
}
