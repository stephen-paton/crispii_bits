use crate::{
    enums::{
        Bin,
        PosU64,
    },
    traits::Set,
};

impl Set for u64 {
    type Pos = PosU64;

    fn set_bit(self, pos_u64: PosU64, bin: Bin) -> Self {
        if bin == Bin::B1 {
            self | (1 << u8::from(pos_u64))
        } else {
            self & !(1 << u8::from(pos_u64))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_60_on() {
        let result: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000.set_bit(PosU64::B60, Bin::B1);

        assert_eq!(result, 0b0001_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    fn bit_61_off() {
        let result: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111.set_bit(PosU64::B61, Bin::B0);

        assert_eq!(result, 0b1101_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111);
    }
}
