use crate::{
    enums::PosU32,
    traits::AddWithCarry,
};

impl AddWithCarry for u32 {
    type Pos = PosU32;

    fn add_with_carry(self, other: Self) -> (Self, Vec<PosU32>) {
        let mut result: u32 = 0;
        let mut carry_bit: u8 = 0;
        
        let mut carried_bits: Vec<PosU32> = Vec::new();

        for i in 0..32 {
            let self_bit = (self >> i) & 1;
            let other_bit = (other >> i) & 1;

            let sum = self_bit + other_bit + (carry_bit as u32);

            if sum > 1 {
                carried_bits.push(PosU32::try_from(i as u8).expect("i is guaranteed to be within the legal range of PosU32"));

                carry_bit = 1;
            } else {
                carry_bit = 0;
            }

            if sum % 2 == 1 {
                result |= 1 << i;
            }
        }

        (result, carried_bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_add_max() {
        let (result, carried_bits) = 0.add_with_carry(u32::MAX);

        assert_eq!(result, u32::MAX);
        assert_eq!(carried_bits.len(), 0);
    }

    #[test]
    fn max_add_one() {
        let (result, carried_bits) = u32::MAX.add_with_carry(1);

        assert_eq!(result, 0);

        let expected_carried_bits = vec![
            PosU32::B0,
            PosU32::B1,
            PosU32::B2,
            PosU32::B3,
            PosU32::B4,
            PosU32::B5,
            PosU32::B6,
            PosU32::B7,
            PosU32::B8,
            PosU32::B9,
            PosU32::B10,
            PosU32::B11,
            PosU32::B12,
            PosU32::B13,
            PosU32::B14,
            PosU32::B15,
            PosU32::B16,
            PosU32::B17,
            PosU32::B18,
            PosU32::B19,
            PosU32::B20,
            PosU32::B21,
            PosU32::B22,
            PosU32::B23,
            PosU32::B24,
            PosU32::B25,
            PosU32::B26,
            PosU32::B27,
            PosU32::B28,
            PosU32::B29,
            PosU32::B30,
            PosU32::B31,
        ];

        for &expected_carried_bit in &expected_carried_bits {
            assert!(carried_bits.contains(&expected_carried_bit), "Expected carried bit '{expected_carried_bit}' not found");
        }
    }
}
