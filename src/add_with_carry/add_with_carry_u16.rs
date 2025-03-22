use crate::PosU16;

pub trait AddWithCarryU16 { 
    fn add_with_carry(self, other: u16) -> (u16, Vec<PosU16>);
}

impl AddWithCarryU16 for u16 {
    fn add_with_carry(self, other: u16) -> (u16, Vec<PosU16>) {
        let mut result: u16 = 0;
        let mut carry_bit: u8 = 0;
        
        let mut carried_bits: Vec<PosU16> = Vec::new();

        for i in 0..16 {
            let self_bit = (self >> i) & 1;
            let other_bit = (other >> i) & 1;

            let sum = self_bit + other_bit + (carry_bit as u16);

            if sum > 1 {
                carried_bits.push(PosU16::try_from(i as u8).expect("i is guaranteed to be within the legal range of PosU16"));

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
        let (result, carried_bits) = 0.add_with_carry(u16::MAX);

        assert_eq!(result, u16::MAX);
        assert_eq!(carried_bits.len(), 0);
    }

    #[test]
    fn max_add_one() {
        let (result, carried_bits) = u16::MAX.add_with_carry(1);

        assert_eq!(result, 0);

        let expected_carried_bits = vec![
            PosU16::B0,
            PosU16::B1,
            PosU16::B2,
            PosU16::B3,
            PosU16::B4,
            PosU16::B5,
            PosU16::B6,
            PosU16::B7,
            PosU16::B8,
            PosU16::B9,
            PosU16::B10,
            PosU16::B11,
            PosU16::B12,
            PosU16::B13,
            PosU16::B14,
            PosU16::B15,
        ];

        for &expected_carried_bit in &expected_carried_bits {
            assert!(carried_bits.contains(&expected_carried_bit), "Expected carried bit '{expected_carried_bit}' not found");
        }
    }
}
