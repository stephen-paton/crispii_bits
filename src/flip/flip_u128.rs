use crate::PosU128;

pub trait FlipU128 { 
    fn flip_bit(self, pos_u128: PosU128) -> Self;
}

impl FlipU128 for u128 {
    fn flip_bit(self, pos_u128: PosU128) -> Self {
        self ^ (1 << u8::from(pos_u128))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_120_flip_on() {
        let result = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000.flip_bit(PosU128::B120);

        assert_eq!(result, 0b0000_0001_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    fn bit_121_flip_off() {
        let result = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111.flip_bit(PosU128::B121);

        assert_eq!(result, 0b1111_1101_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111);
    }
}
