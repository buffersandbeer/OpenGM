use std::result::Result;
use sqlite;

/// Create all NPC database tables.
///
/// # Examples
/// ```
/// use opengm::database::manual_connect;
/// use opengm::database::npc;
///
/// let connection = manual_connect(":memory:".to_string()).unwrap();
///
/// npc::create_tables(connection).unwrap();
/// ```
pub fn create_tables(connection: sqlite::Connection) -> Result<(), String> {
    let npc_name_create = "CREATE TABLE npc_names ( \
                            id INTEGER PRIMARY KEY, \
                            name VARCHAR NOT NULL, \
                            position INTEGER NOT NULL, \
                            origin VARCHAR NOT NULL, \
                            race VARCHAR NOT NULL, \
                            gender VARCHAR NOT NULL);";
    match connection.execute(npc_name_create) {
        Ok(_) => (),
        Err(err) => return Err(err.to_string())
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::manual_connect;
    use super::create_tables;

    #[test]
    fn test_create_tables() {
        let conn = manual_connect(":memory:".to_string()).unwrap();
        match create_tables(conn) {
            Ok(_) => assert!(true),
            Err(err) => assert!(false, err.to_string())
        }
    }
}
