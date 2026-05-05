pub mod en;
pub mod de;
pub mod error;

pub use crate::de::{read_value, from_bytes};


#[cfg(test)]
mod tests {
    use super::*;
    use binserde::Encode;
    
}
