use dotenv::dotenv;
use std::env;
use std::result::Result;
use sqlite;

/// NPC database functions
pub mod npc;

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
pub fn connect() -> Result<sqlite::Connection, String> {
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

pub fn manual_connect(database_url: String) -> Result<sqlite::Connection, String> {
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
