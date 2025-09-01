use crate::traits::UnsignedInteger;

#[allow(private_bounds)]
pub trait AddWithCarry : UnsignedInteger {
    type Pos;

    fn add_with_carry(self, other: Self) -> (Self, Vec<Self::Pos>);
}
