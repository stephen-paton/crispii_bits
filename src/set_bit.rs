use crispii_errors::{CrispiiError, InvalidArgumentError};

pub trait SetBit: Sized { 
    fn set_bit(self, position: u8, on: bool) -> Result<Self, Box<dyn CrispiiError>>;
}

impl SetBit for u8 {
    fn set_bit(mut self, position: u8, on: bool) -> Result<Self, Box<dyn CrispiiError>> {
        let max_position: u8 = 8 - 1;

        if position > max_position {
            return Err(Box::new(InvalidArgumentError::new("position", format!("'{position}' must be between 0 and {max_position} (inclusive)").as_str())));
        }

        self = if on {
            self | (1 << position)
        } else {
            self & !(1 << position)
        };

        Ok(self)
    }
}

impl SetBit for u16 {
    fn set_bit(mut self, position: u8, on: bool) -> Result<Self, Box<dyn CrispiiError>> {
        let max_position: u8 = 16 - 1;

        if position > max_position {
            return Err(Box::new(InvalidArgumentError::new("position", format!("'{position}' must be between 0 and {max_position} (inclusive)").as_str())));
        }

        self = if on {
            self | (1 << position)
        } else {
            self & !(1 << position)
        };

        Ok(self)
    }
}

impl SetBit for u32 {
    fn set_bit(mut self, position: u8, on: bool) -> Result<Self, Box<dyn CrispiiError>> {
        let max_position: u8 = 32 - 1;

        if position > max_position {
            return Err(Box::new(InvalidArgumentError::new("position", format!("'{position}' must be between 0 and {max_position} (inclusive)").as_str())));
        }

        self = if on {
            self | (1 << position)
        } else {
            self & !(1 << position)
        };

        Ok(self)
    }
}

impl SetBit for u64 {
    fn set_bit(mut self, position: u8, on: bool) -> Result<Self, Box<dyn CrispiiError>> {
        let max_position: u8 = 64 - 1;

        if position > max_position {
            return Err(Box::new(InvalidArgumentError::new("position", format!("'{position}' must be between 0 and {max_position} (inclusive)").as_str())));
        }

        self = if on {
            self | (1 << position)
        } else {
            self & !(1 << position)
        };

        Ok(self)
    }
}

impl SetBit for u128 {
    fn set_bit(mut self, position: u8, on: bool) -> Result<Self, Box<dyn CrispiiError>> {
        let max_position: u8 = 128 - 1;

        if position > max_position {
            return Err(Box::new(InvalidArgumentError::new("position", format!("'{position}' must be between 0 and {max_position} (inclusive)").as_str())));
        }

        self = if on {
            self | (1 << position)
        } else {
            self & !(1 << position)
        };

        Ok(self)
    }
}

impl SetBit for usize {
    fn set_bit(mut self, position: u8, on: bool) -> Result<Self, Box<dyn CrispiiError>> {
        let max_position = ((std::mem::size_of::<usize>() * 8) - 1) as u8;

        if position > max_position {
            return Err(Box::new(InvalidArgumentError::new("position", format!("'{position}' must be between 0 and {max_position} (inclusive)").as_str())));
        }

        self = if on {
            self | (1 << position)
        } else {
            self & !(1 << position)
        };

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // u8
    #[test]
    fn u8_on_valid() {
        let result: u8 = 0b0000_0000.set_bit(4, true).unwrap();

        assert_eq!(result, 0b0001_0000);
    }

    #[test]
    fn u8_off_valid() {
        let result: u8 = 0b1111_1111.set_bit(6, false).unwrap();

        assert_eq!(result, 0b1011_1111);
    }

    #[test]
    fn u8_bounds() {
        let result: u8 = 0b0000_0000.set_bit(7, true).unwrap();

        assert_eq!(result, 0b1000_0000);
    }

    #[test]
    #[should_panic]
    fn u8_beyond_bounds() {
        (0b0000_0000 as u8).set_bit(8, true).unwrap();
    }

    // u16
    #[test]
    fn u16_on_valid() {
        let result: u16 = 0b0000_0000_0000_0000.set_bit(10, true).unwrap();

        assert_eq!(result, 0b0000_0100_0000_0000);
    }

    #[test]
    fn u16_off_valid() {
        let result: u16 = 0b1111_1111_1111_1111.set_bit(12, false).unwrap();

        assert_eq!(result, 0b1110_1111_1111_1111);
    }

    #[test]
    fn u16_bounds() {
        let result: u16 = 0b0000_0000_0000_0000.set_bit(15, true).unwrap();

        assert_eq!(result, 0b1000_0000_0000_0000);
    }

    #[test]
    #[should_panic]
    fn u16_beyond_bounds() {
        (0b0000_0000_0000_0000 as u16).set_bit(16, true).unwrap();
    }

    // u32
    #[test]
    fn u32_on_valid() {
        let result: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000.set_bit(23, true).unwrap();

        assert_eq!(result, 0b0000_0000_1000_0000_0000_0000_0000_0000);
    }

    #[test]
    fn u32_off_valid() {
        let result: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111.set_bit(25, false).unwrap();

        assert_eq!(result, 0b1111_1101_1111_1111_1111_1111_1111_1111);
    }

    #[test]
    fn u32_bounds() {
        let result: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000.set_bit(31, true).unwrap();

        assert_eq!(result, 0b1000_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    #[should_panic]
    fn u32_beyond_bounds() {
        (0b0000_0000_0000_0000_0000_0000_0000_0000 as u32).set_bit(32, true).unwrap();
    }

    // u64
    #[test]
    fn u64_on_valid() {
        let result: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000.set_bit(50, true).unwrap();

        assert_eq!(result, 0b0000_0000_0000_0100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    fn u64_off_valid() {
        let result: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111.set_bit(53, false).unwrap();

        assert_eq!(result, 0b1111_1111_1101_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111);
    }

    #[test]
    fn u64_bounds() {
        let result: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000.set_bit(63, true).unwrap();

        assert_eq!(result, 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    #[should_panic]
    fn u64_beyond_bounds() {
        (0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000 as u64).set_bit(64, true).unwrap();
    }

    // u128
    #[test]
    fn u128_on_valid() {
        let result: u128 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000.set_bit(120, true).unwrap();

        assert_eq!(result, 0b0000_0001_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    fn u128_off_valid() {
        let result: u128 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111.set_bit(123, false).unwrap();

        assert_eq!(result, 0b1111_0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111);
    }

    #[test]
    fn u128_bounds() {
        let result: u128 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000.set_bit(127, true).unwrap();

        assert_eq!(result, 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    #[should_panic]
    fn u128_beyond_bounds() {
        (0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000 as u128).set_bit(128, true).unwrap();
    }
}
