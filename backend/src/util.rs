//! Utility functions used in different parts throughout the application.

use std::fmt::Debug;

/// Converts anything debuggable into a string.
pub fn debug<T: Debug>(x: T) -> String { format!("{:?}", x) }
