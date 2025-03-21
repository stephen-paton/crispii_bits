use crate::PosU32;

pub trait FlipU32 { 
    fn flip_bit(self, pos_u32: PosU32) -> Self;
}

impl FlipU32 for u32 {
    fn flip_bit(self, pos_u32: PosU32) -> Self {
        self ^ (1 << u8::from(pos_u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_30_flip_on() {
        let result = 0b0000_0000_0000_0000_0000_0000_0000_0000.flip_bit(PosU32::B30);

        assert_eq!(result, 0b0100_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    fn bit_31_flip_off() {
        let result = 0b1111_1111_1111_1111_1111_1111_1111_1111.flip_bit(PosU32::B31);

        assert_eq!(result, 0b0111_1111_1111_1111_1111_1111_1111_1111);
    }
}
