//! Database module
//! 
//! Handles all database operations including:
//! - Schema management and migrations
//! - Connection pooling
//! - CRUD operations

pub mod schema;
pub mod connection;
pub mod repository;

pub use connection::Database;
pub use repository::*;
