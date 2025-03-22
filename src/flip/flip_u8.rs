use crate::PosU8;

pub trait FlipU8 { 
    fn flip_bit(self, pos_u8: PosU8) -> Self;
}

impl FlipU8 for u8 {
    fn flip_bit(self, pos_u8: PosU8) -> Self {
        self ^ (1 << u8::from(pos_u8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_3_flip_off() {
        let result = 0b0000_0000.flip_bit(PosU8::B3);

        assert_eq!(result, 0b0000_1000);
    }

    #[test]
    fn bit_5_flip_off() {
        let result = 0b1111_1111.flip_bit(PosU8::B5);

        assert_eq!(result, 0b1101_1111);
    }
}
