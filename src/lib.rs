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

/// Jobs is for all thing jobs and job-boards for players to encounter.
///
/// This module contains functions and utilities for creating and managing 
/// jobs, which are essentially hooks and tasks that could be found in job
/// boards or similar to be used as hooks to adventures or just things
/// to keep players busy.
pub mod jobs;

/// npc is a module for all things npc characters.
///
/// It contains functions and utilities for creating and managing npcs, 
/// including characters that the players encounter, background characters,
/// or anything between.
pub mod npc;
