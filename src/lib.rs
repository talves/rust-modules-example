#![allow(dead_code, unused_variables)]
mod auth_utils;
mod database;

use crate::auth_utils::models::Credentials;
use crate::database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = crate::database::connect_to_database() {
        crate::auth_utils::login(creds);
    }
}
