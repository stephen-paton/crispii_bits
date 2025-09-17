//! # Overview
//! Bit operations made **safe**.
//! 
//! Useful for situations where you're emulating CPU registers or need to carry out bit-level operations on Rust's native unsigned integer types (usize excluded).
//! 
//! Provides three extension methods for each:
//! - <code>add_with_carry</code> -> Returns a tuple containing the resulting <code>u{int}</code> value, and a <code>Vec</code> of all of the bit positions where carries occurred (the bit that overflowed to the next bit on the left, not the destination bit of the overlow)
//! - <code>set_bit</code> -> Sets the bit on (1) or off (0) at the position specified
//! - <code>flip_bit</code> -> Flips the bit (0 -> 1 || 1 -> 0) at the position specified
//! 
//! Each of these methods uses a respective <code>PosU{int}</code> enum for bit selection, guaranteeing safety, though the <code>try_into</code> method is available on the <code>u8</code> type if desired.
//! 
//! The former approach is less cumbersome in practice:
//! ```
//! use crispii_bits::u8::*;
//! 
//! let result: u8 = 0b0100_0000.flip_bit(PosU8::B6);
//! ```
//! 
//! vs.
//! 
//! ```
//! use crispii_bits::u8::*;
//! 
//! let result: u8 = 0b0100_0000.flip_bit(6.try_into().unwrap());
//! ``` 
//! 
//! # Importing
//! Importing the crate is as simple as importing <code>*</code> from the <code>u{int}</code> module that matches the native <code>u{int}</code> type you want to use.
//! 
//! For example, a <code>u16</code>:
//! ```
//! use crispii_bits::u16::*;
//! ``` 
mod enums;
mod implementations;
mod traits;

pub mod u8;
pub mod u16;
pub mod u32;
pub mod u64;
pub mod u128;
