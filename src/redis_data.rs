use redis::{Commands, Connection};

//todo rename this file

pub fn redis_connection() -> Connection {
    let redis_client = redis::Client::open("redis://127.0.0.1/")
        .unwrap();

    redis_client.get_connection()
        .unwrap()
}

//todo i think this can be nil?
pub fn access_token() -> String {
    let mut connection = redis_connection();

    connection.get("access_token").unwrap()
}

pub fn has_access_token() -> bool {
    let mut connection = redis_connection();

    connection.exists("access_token").unwrap()
}

