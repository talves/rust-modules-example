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

fn authenticate(creds: Credentials) {}
