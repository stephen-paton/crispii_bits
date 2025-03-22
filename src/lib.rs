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

pub mod flip;
pub use flip::FlipU8;
pub use flip::FlipU16;
pub use flip::FlipU32;
pub use flip::FlipU64;
pub use flip::FlipU128;

pub mod add_with_carry;
pub use add_with_carry::AddWithCarryU8;
pub use add_with_carry::AddWithCarryU16;
pub use add_with_carry::AddWithCarryU32;
pub use add_with_carry::AddWithCarryU64;
pub use add_with_carry::AddWithCarryU128;

pub mod u8;
pub mod u16;
pub mod u32;
pub mod u64;
pub mod u128;
