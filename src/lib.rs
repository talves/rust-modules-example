#![allow(dead_code, unused_variables)]
mod database {

    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        return Status::Connected;
    }

    pub fn get_user() {
        //get user from db
    }
}

mod auth_utils {

    pub fn login(creds: models::Credentials) {
        // authenticate...
        crate::database::get_user();
    }

    pub fn logout() {
        //logout the user...
    }

    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}

use crate::auth_utils::models::Credentials;
use crate::database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = crate::database::connect_to_database() {
        crate::auth_utils::login(creds);
    }
}
