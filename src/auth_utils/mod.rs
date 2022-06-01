pub fn login(creds: models::Credentials) {
    // authenticate...
    crate::database::get_user();
}

pub fn logout() {
    //logout the user...
}

pub mod models;
