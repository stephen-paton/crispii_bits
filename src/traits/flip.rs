use crate::traits::UnsignedInteger;

#[allow(private_bounds)]
pub trait Flip : UnsignedInteger {
    type Pos;

    fn flip_bit(self, pos: Self::Pos) -> Self;
}
