use std::result::Result;
use rusqlite::{Connection, NO_PARAMS};
use rusqlite::types::ToSql;

/// Create all NPC database tables.
///
/// # Examples
/// ```
/// # use opengm::database::manual_connect;
/// use opengm::npc;
///
/// # let connection = manual_connect(":memory:".to_string()).unwrap();
///
/// npc::create_tables(&connection).unwrap();
/// ```
pub fn create_tables(connection: &Connection) -> Result<(), String> {
    let npc_name_create = "CREATE TABLE npc_names ( \
                            id INTEGER PRIMARY KEY, \
                            name VARCHAR NOT NULL, \
                            position INTEGER NOT NULL, \
                            origin VARCHAR NOT NULL, \
                            race VARCHAR NOT NULL, \
                            gender VARCHAR NOT NULL);";
    match connection.execute(npc_name_create, NO_PARAMS) {
        Ok(_) => (),
        Err(err) => return Err(err.to_string())
    };
    Ok(())
}

/// An instance of a name is represented here.
///
/// First name is considered a different instance than a last name
#[derive(Debug, Clone)]
pub struct Name {
    pub name: String,
    pub gender: String,
    pub origin: String,
    pub position: i32,
    pub race: String,
}

impl Name {
    /// Returns a name with the information given
    ///
    /// # Argumetns
    ///
    /// * `name` - The actual value of the name
    pub fn new(name: &str, position: i32, origin: &str, race: &str, gender: &str) -> Name {
        Name {
            name: name.to_string(),
            position: position,
            origin: race.to_string(),
            gender: gender.to_string(),
        }
    }
}

/// Add a NPC name to the database.
///
/// # Examples
/// ```
/// use opengm::database::manual_connect;
/// use opengm::npc;
///
/// let connection = manual_connect(":memory:".to_string()).unwrap();
/// npc::create_tables(&connection).unwrap();
///
/// let new_name = npc::Name {
///     name: "Somebody".to_string(),
///     gender: "None".to_string(),
///     origin: "None".to_string(),
///     position: 0,
///     race: "None".to_string()
/// };
///
/// npc::add_name(&connection, new_name).unwrap();
/// ```
pub fn add_name(connection: &Connection, name: Name) -> Result<(), String> {

    let insert_query = "INSERT INTO npc_names (name, position, origin, race, gender) \
                        VALUES (?1, ?2, ?3, ?4, ?5);";

    match connection.execute(insert_query, &[&name.name, &name.position as &ToSql,
                                             &name.origin, &name.race, &name.gender]) {
        Ok(_) => (),
        Err(err) => return Err(err.to_string())
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::database::manual_connect;
    use super::{create_tables, add_name, Name};
    use rusqlite::{NO_PARAMS};
    use std::collections::HashSet;

    #[test]
    fn test_create_tables() {
       
        let mut result = HashSet::new();
        let mut expected = HashSet::new();
        expected.insert("npc_names");

        let conn = manual_connect(":memory:".to_string()).unwrap();

        match create_tables(&conn) {
            Ok(_) => (),
            Err(err) => assert!(false, err.to_string())
        };

        let test_query = "SELECT name FROM sqlite_master WHERE type='table' and name='npc_names'";
        let mut stmt = conn.prepare(test_query).unwrap();
        let rows = stmt.query_map(NO_PARAMS, |row| row.get::<_, String>(0)).unwrap();
        for row in rows {
            result.insert(row.unwrap());
        }

        for item in expected {
            if !result.contains(item) {
                assert!(false, "Item was missing from results: {}", item);
            }
        }
    }

    #[test]
    fn test_add_name() {
        let conn = manual_connect(":memory:".to_string()).unwrap();
        create_tables(&conn).unwrap();
        
        let new_name = Name {
            name: "Somebody".to_string(),
            gender: "None".to_string(),
            origin: "None".to_string(),
            position: 0,
            race: "None".to_string()
        };
        let expected = new_name.clone();
        add_name(&conn, new_name).unwrap();

        let test_query = "SELECT name, position, origin, race, gender FROM npc_names WHERE name='Somebody'";
        let mut stmt = conn.prepare(test_query).unwrap();
        let name_iter = stmt.query_map(NO_PARAMS, |row| Name {
            name: row.get(0),
            position: row.get(1),
            origin: row.get(2),
            race: row.get(3),
            gender: row.get(4),
        }).unwrap();

        for name in name_iter {
            assert_eq!(name.unwrap().name, expected.name, "The names did not match");
        }
        
    }
}
