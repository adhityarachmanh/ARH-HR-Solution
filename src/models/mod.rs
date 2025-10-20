// src/models/mod.rs
pub mod user;
pub mod role; 
pub mod permission; 

pub use user::User;
pub use role::Role;
pub use permission::{HardcodedPermission};