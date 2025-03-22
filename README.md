# crispii_bits

Bit related functionality intended for use with Crispii

Provides extension methods to Rust's native unsigned int types for bit-level manipulations.

The recommended approach for imports is to use the associated helper crate for the particular int variant you're using.

For example, if you're using u8 values:
```rust
use crispii_bits::u8::*;

fn main() {
    let mut register_a: u8 = 0b0000_0000;

    register_a = register_a.flip_bit(PosU8::B2); // 0b0000_0100

    register_a = register_a.set_bit(PosU8::B7, Bin::B1); // 0b1000_0100

    let (result, carried_bits) = register_a.add_with_carry(0b0000_0100);

    if carried_bits.contains(&PosU8::B2) {
        println!("Addition operation resulted in a carry on bit 2");
    }
}
```

License: MIT OR Apache-2.0
