//! A layer of a neural network.

use crate::array::*;

/// A layer of a neural network, which implements a forward, and backward pass.
pub trait Layer {
    /// Computes the forward pass of the layer.
    fn forward(&self, input: Array) -> Array;

    /// Retrieves the parameters of the layer.
    fn parameters(&mut self) -> Vec<&mut Array>;
}
