use crate::{
    enums::{
        Bin,
        PosU8,
    },
    traits::Set,
};

impl Set for u8 {
    type Pos = PosU8;

    fn set_bit(self, pos_u8: PosU8, bin: Bin) -> Self {
        if bin == Bin::B1 {
            self | (1 << u8::from(pos_u8))
        } else {
            self & !(1 << u8::from(pos_u8))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_3_on() {
        let result: u8 = 0b0000_0000.set_bit(PosU8::B3, Bin::B1);

        assert_eq!(result, 0b0000_1000);
    }

    #[test]
    fn bit_5_off() {
        let result: u8 = 0b1111_1111.set_bit(PosU8::B5, Bin::B0);

        assert_eq!(result, 0b1101_1111);
    }
}
