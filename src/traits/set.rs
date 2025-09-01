use crate::{
    enums::Bin,
    traits::UnsignedInteger,
};

#[allow(private_bounds)]
pub trait Set : UnsignedInteger {
    type Pos;

    fn set_bit(self, pos: Self::Pos, bin: Bin) -> Self;
}
