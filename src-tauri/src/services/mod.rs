//! Services module
//! 
//! Business logic services

pub mod file_scanner;

pub use file_scanner::{FileScanner, ScannerConfig, ScanStats};
