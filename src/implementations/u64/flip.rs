use crate::{
    enums::PosU64,
    traits::Flip,
};

impl Flip for u64 {
    type Pos = PosU64;

    fn flip_bit(self, pos_u64: PosU64) -> Self {
        self ^ (1 << u8::from(pos_u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_60_flip_on() {
        let result: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000.flip_bit(PosU64::B60);

        assert_eq!(result, 0b0001_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    fn bit_61_off() {
        let result: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111.flip_bit(PosU64::B61);

        assert_eq!(result, 0b1101_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111);
    }
}
