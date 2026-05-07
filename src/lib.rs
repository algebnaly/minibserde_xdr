pub mod de;
pub mod en;
pub mod error;

pub use crate::de::{decode_len, from_bytes, read_value};
pub use crate::en::{to_bytes, write_value};
pub use error::Error;

#[cfg(test)]
mod tests {} // TODO: add more tests
