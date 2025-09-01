use std::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum PosU8 {
    #[default]
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
}

impl Display for PosU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_u8 = u8::from(*self);

        write!(f, "u8:Bit {as_u8}")
    }
}

impl From<PosU8> for u8 {
    fn from(value: PosU8) -> Self {
        match value {
            PosU8::B0 => 0,
            PosU8::B1 => 1,
            PosU8::B2 => 2,
            PosU8::B3 => 3,
            PosU8::B4 => 4,
            PosU8::B5 => 5,
            PosU8::B6 => 6,
            PosU8::B7 => 7,
        }
    }
}

impl TryFrom<u8> for PosU8 {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PosU8::B0),
            1 => Ok(PosU8::B1),
            2 => Ok(PosU8::B2),
            3 => Ok(PosU8::B3),
            4 => Ok(PosU8::B4),
            5 => Ok(PosU8::B5),
            6 => Ok(PosU8::B6),
            7 => Ok(PosU8::B7),
            _ => Err("PosU8 can only be represented by the u8 values 0 to 7 (inclusive)!")
        }
    }
}
