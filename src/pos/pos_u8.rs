pub enum PosU8 {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
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
