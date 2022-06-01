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

    mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }

    pub fn authenticate(creds: models::Credentials) {
        if let crate::database::Status::Connected = crate::database::connect_to_database() {
            login(creds);
        }
    }
}
