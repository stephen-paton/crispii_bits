use std::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum PosU16 {
    #[default]
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    B10,
    B11,
    B12,
    B13,
    B14,
    B15,
}

impl Display for PosU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_u8 = u8::from(*self);

        write!(f, "u16:Bit {as_u8}")
    }
}

impl From<PosU16> for u8 {
    fn from(value: PosU16) -> Self {
        match value {
            PosU16::B0 => 0,
            PosU16::B1 => 1,
            PosU16::B2 => 2,
            PosU16::B3 => 3,
            PosU16::B4 => 4,
            PosU16::B5 => 5,
            PosU16::B6 => 6,
            PosU16::B7 => 7,
            PosU16::B8 => 8,
            PosU16::B9 => 9,
            PosU16::B10 => 10,
            PosU16::B11 => 11,
            PosU16::B12 => 12,
            PosU16::B13 => 13,
            PosU16::B14 => 14,
            PosU16::B15 => 15,
        }
    }
}

impl TryFrom<u8> for PosU16 {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PosU16::B0),
            1 => Ok(PosU16::B1),
            2 => Ok(PosU16::B2),
            3 => Ok(PosU16::B3),
            4 => Ok(PosU16::B4),
            5 => Ok(PosU16::B5),
            6 => Ok(PosU16::B6),
            7 => Ok(PosU16::B7),
            8 => Ok(PosU16::B8),
            9 => Ok(PosU16::B9),
            10 => Ok(PosU16::B10),
            11 => Ok(PosU16::B11),
            12 => Ok(PosU16::B12),
            13 => Ok(PosU16::B13),
            14 => Ok(PosU16::B14),
            15 => Ok(PosU16::B15),
            _ => Err("PosU16 can only be represented by the u8 values 0 to 15 (inclusive)!")
        }
    }
}
