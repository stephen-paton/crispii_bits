use crate::PosU64;

pub trait AddWithCarryU64 { 
    fn add_with_carry(self, other: u64) -> (u64, Vec<PosU64>);
}

impl AddWithCarryU64 for u64 {
    fn add_with_carry(self, other: u64) -> (u64, Vec<PosU64>) {
        let mut result: u64 = 0;
        let mut carry_bit: u8 = 0;
        
        let mut carried_bits: Vec<PosU64> = Vec::new();

        for i in 0..64 {
            let self_bit = (self >> i) & 1;
            let other_bit = (other >> i) & 1;

            let sum = self_bit + other_bit + (carry_bit as u64);

            if sum > 1 {
                carried_bits.push(PosU64::try_from(i as u8).expect("i is guaranteed to be within the legal range of PosU64"));

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
        let (result, carried_bits) = 0.add_with_carry(u64::MAX);

        assert_eq!(result, u64::MAX);
        assert_eq!(carried_bits.len(), 0);
    }

    #[test]
    fn max_add_one() {
        let (result, carried_bits) = u64::MAX.add_with_carry(1);

        assert_eq!(result, 0);

        let expected_carried_bits = vec![
            PosU64::B0,
            PosU64::B1,
            PosU64::B2,
            PosU64::B3,
            PosU64::B4,
            PosU64::B5,
            PosU64::B6,
            PosU64::B7,
            PosU64::B8,
            PosU64::B9,
            PosU64::B10,
            PosU64::B11,
            PosU64::B12,
            PosU64::B13,
            PosU64::B14,
            PosU64::B15,
            PosU64::B16,
            PosU64::B17,
            PosU64::B18,
            PosU64::B19,
            PosU64::B20,
            PosU64::B21,
            PosU64::B22,
            PosU64::B23,
            PosU64::B24,
            PosU64::B25,
            PosU64::B26,
            PosU64::B27,
            PosU64::B28,
            PosU64::B29,
            PosU64::B30,
            PosU64::B31,
            PosU64::B32,
            PosU64::B33,
            PosU64::B34,
            PosU64::B35,
            PosU64::B36,
            PosU64::B37,
            PosU64::B38,
            PosU64::B39,
            PosU64::B40,
            PosU64::B41,
            PosU64::B42,
            PosU64::B43,
            PosU64::B44,
            PosU64::B45,
            PosU64::B46,
            PosU64::B47,
            PosU64::B48,
            PosU64::B49,
            PosU64::B50,
            PosU64::B51,
            PosU64::B52,
            PosU64::B53,
            PosU64::B54,
            PosU64::B55,
            PosU64::B56,
            PosU64::B57,
            PosU64::B58,
            PosU64::B59,
            PosU64::B60,
            PosU64::B61,
            PosU64::B62,
            PosU64::B63,
        ];

        for &expected_carried_bit in &expected_carried_bits {
            assert!(carried_bits.contains(&expected_carried_bit), "Expected carried bit '{expected_carried_bit}' not found");
        }
    }
}
