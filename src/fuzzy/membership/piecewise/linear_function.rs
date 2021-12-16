use crate::utilities;
use impl_ops::*;
use std::ops;

/// Linear function struct.
///
/// f(x) = ax + b; a == slope & b == intercept.
#[derive(Debug, PartialEq, Clone)]
pub struct LinearFunction {
    /// Slope.
    slope: f64,

    /// Intercept.
    intercept: f64,
}

impl LinearFunction {
    /// Linear function constructor.
    ///
    /// # Arguments
    /// * `slope`: Function slope.
    /// * `intercept`: Function intercept.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::membership::piecewise::LinearFunction;
    /// LinearFunction::new(2.3, 3.4);
    /// ```
    pub fn new(slope: f64, intercept: f64) -> Self {
        Self {
            slope: utilities::math::round_f64(slope, 5),
            intercept: utilities::math::round_f64(intercept, 5),
        }
    }

    /// Returns slope.
    ///
    /// ```
    /// # use assessment::fuzzy::membership::piecewise::LinearFunction;
    /// let function = LinearFunction::new(2.3, 3.4);
    /// assert_eq!(function.slope(), 2.3);
    /// ```
    pub fn slope(&self) -> f64 {
        self.slope
    }

    /// Returns intercept.
    ///
    /// ```
    /// # use assessment::fuzzy::membership::piecewise::LinearFunction;
    /// let function = LinearFunction::new(2.3, 3.4);
    /// assert_eq!(function.intercept(), 3.4);
    /// ```
    pub fn intercept(&self) -> f64 {
        self.intercept
    }

    /// Sums the current function with `other` function and returns a new function.
    ///
    /// # Arguments
    /// * `other`: Function to sum to the current one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::membership::piecewise::LinearFunction;
    /// let a = LinearFunction::new(2.1, 3.1);
    /// let b = LinearFunction::new(2.3, 3.7);
    ///
    /// let sum_a_a = a.sum(&b);
    /// let sum_a_b = b.sum(&a);
    /// assert_eq!(sum_a_a.slope(), sum_a_b.slope());
    /// assert_eq!(sum_a_a.intercept(), sum_a_b.intercept());
    /// assert_eq!(sum_a_a.slope(), 4.4);
    /// assert_eq!(sum_a_a.intercept(), 6.8);
    /// ```
    pub fn sum(&self, other: &LinearFunction) -> Self {
        LinearFunction::new(self.slope + other.slope, self.intercept + other.intercept)
    }
}

impl_op!(+ |a: &LinearFunction, b: &LinearFunction| -> LinearFunction { a.sum(b) });
impl_op!(+ |a: LinearFunction, b: &LinearFunction| -> LinearFunction { a.sum(b) });
impl_op!(+ |a: &LinearFunction, b: LinearFunction| -> LinearFunction { a.sum(&b) });
impl_op!(+ |a: LinearFunction, b: LinearFunction| -> LinearFunction { a.sum(&b) });

#[cfg(test)]
mod tests {
    use crate::fuzzy::membership::piecewise::LinearFunction;

    #[test]
    fn sum_references() {
        let a = LinearFunction::new(2.1, 3.1);
        let b = LinearFunction::new(2.3, 3.7);

        let sum = &a + &b;
        assert_eq!(sum.slope(), 4.4);
        assert_eq!(sum.intercept(), 6.8);
    }

    #[test]
    fn sum_ownerships() {
        let a = LinearFunction::new(2.1, 3.1);
        let b = LinearFunction::new(2.3, 3.7);

        let sum = a + b;
        assert_eq!(sum.slope(), 4.4);
        assert_eq!(sum.intercept(), 6.8);
    }

    #[test]
    fn sum_ownership_reference() {
        let a = LinearFunction::new(2.1, 3.1);
        let b = LinearFunction::new(2.3, 3.7);

        let sum = a + &b;
        assert_eq!(sum.slope(), 4.4);
        assert_eq!(sum.intercept(), 6.8);
    }

    #[test]
    fn sum_reference_ownership() {
        let a = LinearFunction::new(2.1, 3.1);
        let b = LinearFunction::new(2.3, 3.7);

        let sum = &a + b;
        assert_eq!(sum.slope(), 4.4);
        assert_eq!(sum.intercept(), 6.8);
    }
}
