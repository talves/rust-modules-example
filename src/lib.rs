#![allow(dead_code, unused_variables)]
struct Credentials {
    username: String,
    password: String,
}
enum Status {
    Connected,
    Interrupted,
}

fn connect_to_database() -> Status {
    return Status::Connected;
}

fn get_user() {
    //get user from db
}

fn login(creds: Credentials) {
    // authenticate...
    get_user();
}

fn logout() {
    //logout the user...
}

fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds);
    }
}
