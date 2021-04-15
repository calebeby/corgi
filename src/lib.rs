#![warn(missing_docs)]

//! Machine learning, and dynamic automatic differentiation implementation.

#[cfg(feature = "blas")]
extern crate libc;

pub mod numbers;
#[macro_use]
pub mod array;
#[cfg(feature = "blas")]
pub mod blas;
pub mod layer;
pub mod layers;
pub mod model;
pub mod nn_functions;
pub mod optimizer;
pub mod optimizers;

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use array::*;
    use numbers::*;

    #[test]
    fn test_op() {
        let op: array::ForwardOp = Arc::new(|x: &[&Array]| {
            Arrays::new((
                x[0].dimensions(),
                x[0].values()
                    .iter()
                    .zip(x[1].values())
                    .map(|(x, y)| x * y)
                    .collect::<Vec<Float>>(),
            ))
        });

        let op_clone = Arc::clone(&op);
        let backward_op: array::BackwardOp = Arc::new(move |c: &mut Vec<Array>, x: &Array| {
            vec![
                Some(Array::op(&vec![&c[1], x], Arc::clone(&op_clone), None)),
                Some(Array::op(&vec![&c[0], x], Arc::clone(&op_clone), None)),
            ]
        });

        let a = arr![1.0, 2.0, 3.0];
        let b = arr![3.0, 2.0, 1.0];
        let mut product = Array::op(&vec![&a, &b], op, Some(backward_op));
        assert_eq!(product, arr![3.0, 4.0, 3.0]);
        product.backward(None);
        assert_eq!(product.gradient().unwrap(), arr![1.0, 1.0, 1.0]);
        assert_eq!(b.gradient().unwrap(), arr![1.0, 2.0, 3.0]);
        assert_eq!(a.gradient().unwrap(), arr![3.0, 2.0, 1.0]);
    }
}
