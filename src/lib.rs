//! Bit related functionality intended for use with Crispii
//! 
//! Provides extension methods to Rust's native unsigned int types for bit-level manipulations.
//! 
//! The recommended approach for imports is to use the associated helper crate for the particular int variant you're using.
//! 
//! For example, if you're using u8 values:
//! ```
//! use crispii_bits::u8::*;
//! 
//! fn main() {
//!     let mut register_a: u8 = 0b0000_0000;
//! 
//!     register_a = register_a.flip_bit(PosU8::B2); // 0b0000_0100
//!     
//!     register_a = register_a.set_bit(PosU8::B7, Bin::B1); // 0b1000_0100
//!     
//!     let (result, carried_bits) = register_a.add_with_carry(0b0000_0100);
//! 
//!     if carried_bits.contains(&PosU8::B2) {
//!         println!("Addition operation resulted in a carry on bit 2");
//!     }
//! }
//! ```
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
