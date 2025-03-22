use crate::PosU8;

pub trait AddWithCarryU8 { 
    fn add_with_carry(self, other: u8) -> (u8, Vec<PosU8>);
}

impl AddWithCarryU8 for u8 {
    fn add_with_carry(self, other: u8) -> (u8, Vec<PosU8>) {
        let mut result: u8 = 0;
        let mut carry_bit: u8 = 0;
        
        let mut carried_bits: Vec<PosU8> = Vec::new();

        for i in 0..8 {
            let self_bit = (self >> i) & 1;
            let other_bit = (other >> i) & 1;

            let sum = self_bit + other_bit + carry_bit;

            if sum > 1 {
                carried_bits.push(PosU8::try_from(i as u8).expect("i is guaranteed to be within the legal range of PosU8"));

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
        let (result, carried_bits) = 0.add_with_carry(u8::MAX);

        assert_eq!(result, u8::MAX);
        assert_eq!(carried_bits.len(), 0);
    }

    #[test]
    fn max_add_one() {
        let (result, carried_bits) = u8::MAX.add_with_carry(1);

        assert_eq!(result, 0);

        let expected_carried_bits = vec![
            PosU8::B0,
            PosU8::B1,
            PosU8::B2,
            PosU8::B3,
            PosU8::B4,
            PosU8::B5,
            PosU8::B6,
            PosU8::B7,
        ];

        for &expected_carried_bit in &expected_carried_bits {
            assert!(carried_bits.contains(&expected_carried_bit), "Expected carried bit '{expected_carried_bit}' not found");
        }
    }
}
