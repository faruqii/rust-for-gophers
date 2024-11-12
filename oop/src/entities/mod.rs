/*
Mod.rs is a file that tells Rust that the entities folder is a module.
This allows us to group related files together and use them in other parts of the code.
*/

pub mod car;

pub use car::{Car, Engine};
