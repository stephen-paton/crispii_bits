pub enum PosU16 {
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
