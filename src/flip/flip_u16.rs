use crate::PosU16;

pub trait FlipU16 { 
    fn flip_bit(self, pos_u16: PosU16) -> Self;
}

impl FlipU16 for u16 {
    fn flip_bit(self, pos_u16: PosU16) -> Self {
        self ^ (1 << u8::from(pos_u16))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_12_flip_on() {
        let result = 0b0000_0000_0000_0000.flip_bit(PosU16::B12);

        assert_eq!(result, 0b0001_0000_0000_0000);
    }

    #[test]
    fn bit_14_flip_off() {
        let result = 0b1111_1111_1111_1111.flip_bit(PosU16::B14);

        assert_eq!(result, 0b1011_1111_1111_1111);
    }
}
