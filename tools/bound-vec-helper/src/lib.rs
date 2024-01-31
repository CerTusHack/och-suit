#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{BoundedVec, traits::Get};
use sp_std::{vec::Vec, fmt::Debug, result::Result};
use sp_std::convert::TryFrom;

pub trait BoundVecHelper<T, S> {
    fn create_on_vec(v: Vec<T>) -> Result<Self, <Self as TryFrom<Vec<T>>>::Error> where Self: Sized;
    fn check_push(&mut self, v: T) -> Result<(), &'static str>;
}

impl<T, S> BoundVecHelper<T, S> for BoundedVec<T, S>
where
    S: Get<u32>,
    BoundedVec<T, S>: Debug + TryFrom<Vec<T>>,
{
    fn create_on_vec(v: Vec<T>) -> Result<Self, <Self as TryFrom<Vec<T>>>::Error> {
        Self::try_from(v)
    }

    fn check_push(&mut self, v: T) -> Result<(), &'static str> {
        if self.try_push(v).is_err() {
            Err("BoundedVec is already at maximum capacity")
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use frame_support::{BoundedVec, traits::ConstU32};
    use super::BoundVecHelper;

    #[test]
    fn test_boundvec() {
        type TestBoundVec = BoundedVec<u8, ConstU32<3>>;

        let origin_v = vec![b'A', b'B'];
        let mut origin_b = TestBoundVec::create_on_vec(origin_v).unwrap();
        assert_eq!(origin_b.len(), 2);
        
        origin_b.check_push(b'C").expect("Push failed");
        assert_eq!(origin_b.len(), 3);
        assert_eq!(origin_b[0], b'A');
        assert_eq!(origin_b[1], b'B');
        assert_eq!(origin_b[2], b'C');
    }
}