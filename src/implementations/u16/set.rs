use crate::{
    enums::{
        Bin,
        PosU16,
    },
    traits::Set,
};

impl Set for u16 {
    type Pos = PosU16;

    fn set_bit(self, pos_u16: PosU16, bin: Bin) -> Self {
        if bin == Bin::B1 {
            self | (1 << u8::from(pos_u16))
        } else {
            self & !(1 << u8::from(pos_u16))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_12_on() {
        let result: u16 = 0b0000_0000_0000_0000.set_bit(PosU16::B12, Bin::B1);

        assert_eq!(result, 0b0001_0000_0000_0000);
    }

    #[test]
    fn bit_14_off() {
        let result: u16 = 0b1111_1111_1111_1111.set_bit(PosU16::B14, Bin::B0);

        assert_eq!(result, 0b1011_1111_1111_1111);
    }
}
