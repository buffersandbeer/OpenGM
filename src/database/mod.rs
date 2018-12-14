use dotenv::dotenv;
use std::env;
use std::result::Result;
use rusqlite::{Connection};


/// Connects to a Sqlite Database from environment.
///
/// This will read the __OPENGM_DATABASE_URL__ environment variable from 
/// either the system environment, or the .env file at the project root.
///
/// # Examples
/// ```
/// use opengm::database::connect;
///
/// let connection = connect();
/// ```
pub fn connect() -> Result<Connection, String> {
    dotenv().ok();

    let database_url = match env::var("OPENGM_DATABASE_URL") {
        Ok(database_url) => database_url,
        Err(err) => return Err(err.to_string()),
    };
    let connection = match manual_connect(database_url) {
        Ok(connection) => connection,
        Err(err) => return Err(err.to_string())
    };
    return Ok(connection);
}


/// Connects to a Sqlite Database using a given connect string.
///
/// This is for manual connections to the database, usually for testing purposes. 
/// The database::connect function wraps around this for more automatic connection 
/// to the application database.
///
/// # Examples
/// ```
/// use opengm::database::manual_connect;
///
/// let connection = manual_connect(":memory:".to_string()).unwrap();
/// ```
pub fn manual_connect(database_url: String) -> Result<Connection, String> {
    let connection = match Connection::open(database_url) {
        Ok(connection) => connection,
        Err(err) => return Err(err.to_string())
    };
    return Ok(connection);

}


#[cfg(test)]
mod tests {
    use database;
    #[test]
    fn test_connect() {
        match database::connect() {
            Ok(_) => assert!(true),
            Err(err) => assert!(false, err.to_string())
        };
    }

    #[test]
    fn test_manual_connect() {
        match database::manual_connect(":memory:".to_string()) {
            Ok(_) => assert!(true),
            Err(err) => assert!(false, err.to_string())
        };
    }
}
