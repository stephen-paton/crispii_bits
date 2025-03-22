use std::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum PosU32 {
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
    B16,
    B17,
    B18,
    B19,
    B20,
    B21,
    B22,
    B23,
    B24,
    B25,
    B26,
    B27,
    B28,
    B29,
    B30,
    B31,
}

impl Display for PosU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_u8 = u8::from(*self);

        write!(f, "u32:Bit {as_u8}")
    }
}

impl From<PosU32> for u8 {
    fn from(value: PosU32) -> Self {
        match value {
            PosU32::B0 => 0,
            PosU32::B1 => 1,
            PosU32::B2 => 2,
            PosU32::B3 => 3,
            PosU32::B4 => 4,
            PosU32::B5 => 5,
            PosU32::B6 => 6,
            PosU32::B7 => 7,
            PosU32::B8 => 8,
            PosU32::B9 => 9,
            PosU32::B10 => 10,
            PosU32::B11 => 11,
            PosU32::B12 => 12,
            PosU32::B13 => 13,
            PosU32::B14 => 14,
            PosU32::B15 => 15,
            PosU32::B16 => 16,
            PosU32::B17 => 17,
            PosU32::B18 => 18,
            PosU32::B19 => 19,
            PosU32::B20 => 20,
            PosU32::B21 => 21,
            PosU32::B22 => 22,
            PosU32::B23 => 23,
            PosU32::B24 => 24,
            PosU32::B25 => 25,
            PosU32::B26 => 26,
            PosU32::B27 => 27,
            PosU32::B28 => 28,
            PosU32::B29 => 29,
            PosU32::B30 => 30,
            PosU32::B31 => 31,
        }
    }
}

impl TryFrom<u8> for PosU32 {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PosU32::B0),
            1 => Ok(PosU32::B1),
            2 => Ok(PosU32::B2),
            3 => Ok(PosU32::B3),
            4 => Ok(PosU32::B4),
            5 => Ok(PosU32::B5),
            6 => Ok(PosU32::B6),
            7 => Ok(PosU32::B7),
            8 => Ok(PosU32::B8),
            9 => Ok(PosU32::B9),
            10 => Ok(PosU32::B10),
            11 => Ok(PosU32::B11),
            12 => Ok(PosU32::B12),
            13 => Ok(PosU32::B13),
            14 => Ok(PosU32::B14),
            15 => Ok(PosU32::B15),
            16 => Ok(PosU32::B16),
            17 => Ok(PosU32::B17),
            18 => Ok(PosU32::B18),
            19 => Ok(PosU32::B19),
            20 => Ok(PosU32::B20),
            21 => Ok(PosU32::B21),
            22 => Ok(PosU32::B22),
            23 => Ok(PosU32::B23),
            24 => Ok(PosU32::B24),
            25 => Ok(PosU32::B25),
            26 => Ok(PosU32::B26),
            27 => Ok(PosU32::B27),
            28 => Ok(PosU32::B28),
            29 => Ok(PosU32::B29),
            30 => Ok(PosU32::B30),
            31 => Ok(PosU32::B31),
            _ => Err("PosU32 can only be represented by the u8 values 0 to 31 (inclusive)!")
        }
    }
}
