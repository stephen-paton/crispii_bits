use crate::{PosU32, Bin};

pub trait SetU32 { 
    fn set_bit(self, pos_u32: PosU32, bin: Bin) -> Self;
}

impl SetU32 for u32 {
    fn set_bit(self, pos_u32: PosU32, bin: Bin) -> Self {
        if bin == Bin::B1 {
            self | (1 << u8::from(pos_u32))
        } else {
            self & !(1 << u8::from(pos_u32))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_30_on() {
        let result = 0b0000_0000_0000_0000_0000_0000_0000_0000.set_bit(PosU32::B30, Bin::B1);

        assert_eq!(result, 0b0100_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    fn bit_31_off() {
        let result = 0b1111_1111_1111_1111_1111_1111_1111_1111.set_bit(PosU32::B31, Bin::B0);

        assert_eq!(result, 0b0111_1111_1111_1111_1111_1111_1111_1111);
    }
}
