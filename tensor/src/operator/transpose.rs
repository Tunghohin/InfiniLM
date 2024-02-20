﻿use super::Operator;
use crate::{udim, Affine, Shape};
use smallvec::{smallvec, SmallVec};

type Permutation = Shape;

#[repr(transparent)]
pub struct Transpose(Permutation);

impl Operator for Transpose {
    fn build(&self, input: &[udim]) -> SmallVec<[(Shape, Affine); 1]> {
        debug_assert_eq!(input.len(), self.0.len());
        let shape = self.0.iter().map(|&i| input[i as usize]).collect();
        let n = self.0.len();
        let affine = Affine::from_fn(n + 1, n + 1, |r, c| {
            if c == self.0.get(r).map_or(r, |&p| p as usize) {
                1
            } else {
                0
            }
        });
        smallvec![(shape, affine)]
    }
}

impl Transpose {
    #[inline]
    pub fn new(permutation: &[udim]) -> Self {
        Self(Permutation::from_slice(permutation))
    }
}

#[test]
fn test() {
    let ans = Transpose::new(&[0, 3, 1, 2]).build(&[1, 2, 3, 4]);
    assert_eq!(ans.len(), 1);
    assert_eq!(ans[0].0.as_slice(), &[1, 4, 2, 3]);
    assert_eq!(
        ans[0].1.as_slice(),
        &[
            // column major
            1, 0, 0, 0, 0, //
            0, 0, 1, 0, 0, //
            0, 0, 0, 1, 0, //
            0, 1, 0, 0, 0, //
            0, 0, 0, 0, 1, //
        ]
    );
}
