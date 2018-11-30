use dotenv::dotenv;
use std::env;
use std::result::Result;
use sqlite;

pub fn test() {
    println!("Hello, Database");
}

pub fn connect() -> Result<sqlite::Connection, String> {
    dotenv().ok();

    let database_url = match env::var("OPENGM_DATABASE_URL") {
        Ok(database_url) => database_url,
        Err(err) => return Err(err.to_string()),
    };
    let connection = match sqlite::open(database_url) {
        Ok(connection) => connection,
        Err(err) => return Err(err.to_string())
    };
    return Ok(connection);
}

#[cfg(test)]
mod tests {
    use database::connect;
    #[test]
    fn test_connect() {
        match connect() {
            Ok(_) => assert!(true),
            Err(err) => assert!(false, err.to_string())
        };
    }
}
