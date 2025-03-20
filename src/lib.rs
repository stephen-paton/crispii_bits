//! Bit related functionality intended for use with Crispii
pub use crispii_digits::Bin;

pub mod pos;
pub use pos::PosU8;
pub use pos::PosU16;
pub use pos::PosU32;
pub use pos::PosU64;
pub use pos::PosU128;

pub mod set;
pub use set::SetU8;
pub use set::SetU16;
pub use set::SetU32;
pub use set::SetU64;
pub use set::SetU128;
