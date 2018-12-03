extern crate dotenv;
extern crate rusqlite;

/// The database module is the primary way to use the database
///
/// This module, and any subsequent modules, contain all the 
/// database functions, models, and schemas.
///
/// # Examples
/// ```
/// use opengm::database;
///
/// let connection = database::connect();
/// ```
pub mod database;
