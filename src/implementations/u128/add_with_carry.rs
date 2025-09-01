use crate::{
    enums::PosU128,
    traits::AddWithCarry,
};

impl AddWithCarry for u128 {
    type Pos = PosU128;

    fn add_with_carry(self, other: u128) -> (u128, Vec<PosU128>) {
        let mut result: u128 = 0;
        let mut carry_bit: u8 = 0;
        
        let mut carried_bits: Vec<PosU128> = Vec::new();

        for i in 0..128 {
            let self_bit = (self >> i) & 1;
            let other_bit = (other >> i) & 1;

            let sum = self_bit + other_bit + (carry_bit as u128);

            if sum > 1 {
                carried_bits.push(PosU128::try_from(i as u8).expect("i is guaranteed to be within the legal range of PosU128"));

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
        let (result, carried_bits) = 0.add_with_carry(u128::MAX);

        assert_eq!(result, u128::MAX);
        assert_eq!(carried_bits.len(), 0);
    }

    #[test]
    fn max_add_one() {
        let (result, carried_bits) = u128::MAX.add_with_carry(1);

        assert_eq!(result, 0);

        let expected_carried_bits = vec![
            PosU128::B0,
            PosU128::B1,
            PosU128::B2,
            PosU128::B3,
            PosU128::B4,
            PosU128::B5,
            PosU128::B6,
            PosU128::B7,
            PosU128::B8,
            PosU128::B9,
            PosU128::B10,
            PosU128::B11,
            PosU128::B12,
            PosU128::B13,
            PosU128::B14,
            PosU128::B15,
            PosU128::B16,
            PosU128::B17,
            PosU128::B18,
            PosU128::B19,
            PosU128::B20,
            PosU128::B21,
            PosU128::B22,
            PosU128::B23,
            PosU128::B24,
            PosU128::B25,
            PosU128::B26,
            PosU128::B27,
            PosU128::B28,
            PosU128::B29,
            PosU128::B30,
            PosU128::B31,
            PosU128::B32,
            PosU128::B33,
            PosU128::B34,
            PosU128::B35,
            PosU128::B36,
            PosU128::B37,
            PosU128::B38,
            PosU128::B39,
            PosU128::B40,
            PosU128::B41,
            PosU128::B42,
            PosU128::B43,
            PosU128::B44,
            PosU128::B45,
            PosU128::B46,
            PosU128::B47,
            PosU128::B48,
            PosU128::B49,
            PosU128::B50,
            PosU128::B51,
            PosU128::B52,
            PosU128::B53,
            PosU128::B54,
            PosU128::B55,
            PosU128::B56,
            PosU128::B57,
            PosU128::B58,
            PosU128::B59,
            PosU128::B60,
            PosU128::B61,
            PosU128::B62,
            PosU128::B63,
            PosU128::B64,
            PosU128::B65,
            PosU128::B66,
            PosU128::B67,
            PosU128::B68,
            PosU128::B69,
            PosU128::B70,
            PosU128::B71,
            PosU128::B72,
            PosU128::B73,
            PosU128::B74,
            PosU128::B75,
            PosU128::B76,
            PosU128::B77,
            PosU128::B78,
            PosU128::B79,
            PosU128::B80,
            PosU128::B81,
            PosU128::B82,
            PosU128::B83,
            PosU128::B84,
            PosU128::B85,
            PosU128::B86,
            PosU128::B87,
            PosU128::B88,
            PosU128::B89,
            PosU128::B90,
            PosU128::B91,
            PosU128::B92,
            PosU128::B93,
            PosU128::B94,
            PosU128::B95,
            PosU128::B96,
            PosU128::B97,
            PosU128::B98,
            PosU128::B99,
            PosU128::B100,
            PosU128::B101,
            PosU128::B102,
            PosU128::B103,
            PosU128::B104,
            PosU128::B105,
            PosU128::B106,
            PosU128::B107,
            PosU128::B108,
            PosU128::B109,
            PosU128::B110,
            PosU128::B111,
            PosU128::B112,
            PosU128::B113,
            PosU128::B114,
            PosU128::B115,
            PosU128::B116,
            PosU128::B117,
            PosU128::B118,
            PosU128::B119,
            PosU128::B120,
            PosU128::B121,
            PosU128::B122,
            PosU128::B123,
            PosU128::B124,
            PosU128::B125,
            PosU128::B126,
            PosU128::B127,
        ];

        for &expected_carried_bit in &expected_carried_bits {
            assert!(carried_bits.contains(&expected_carried_bit), "Expected carried bit '{expected_carried_bit}' not found");
        }
    }
}
