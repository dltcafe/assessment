use crate::fuzzy::membership::piecewise::{LinearFunction, PiecewiseLinearFunction};
use std::fmt::{Display, Formatter};

use super::Membership;

/// Trapezoidal Membership Function struct.
///
/// This function is defined by four points going from left to right `(a, b, c, d)`.
///
/// `[a-d]` is the base of the trapezoid and `[b-c]` is the center.
///
/// If `b=c` the function is called **Triangular**.
#[derive(Debug, PartialEq, Clone)]
pub struct Trapezoidal {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

/// Trapezoidal errors types
#[derive(Debug, PartialEq)]
pub enum TrapezoidalError {
    /// Not enough values
    NotEnoughValues { limits: Vec<f32> },
    /// Too many values
    TooManyValues { limits: Vec<f32> },
    /// Unordered values
    UnorderedValues { limits: Vec<f32> },
}

impl Display for TrapezoidalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use TrapezoidalError::*;
        match &self {
            NotEnoughValues { limits } => {
                write!(
                    f,
                    "Trapezoidal membership function needs at least 3 values, you provided {}.",
                    limits.len()
                )
            }
            TooManyValues { limits } => {
                write!(
                    f,
                    "Trapezoidal membership function needs at most 4 values, you provided {}.",
                    limits.len()
                )
            }
            UnorderedValues { limits: _ } => {
                write!(
                    f,
                    "Trapezoidal membership function needs an ordered array of values."
                )
            }
        }
    }
}
impl Membership for Trapezoidal {}

impl Display for Trapezoidal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_triangular() {
            write!(f, "({:.2}, {:.2}, {:.2})", self.a, self.b, self.d)
        } else {
            write!(
                f,
                "({:.2}, {:.2}, {:.2}, {:.2})",
                self.a, self.b, self.c, self.d
            )
        }
    }
}

impl Trapezoidal {
    /// Trapezoidal membership function constructor.
    ///
    /// # Arguments
    /// * `limits`: Membership function limits.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::membership::Trapezoidal;
    /// for (v, e) in [
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2, 0.3]), "(0.00, 0.10, 0.20, 0.30)"),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.1, 0.2]), "(0.00, 0.10, 0.20)"),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2]), "(0.00, 0.10, 0.20)")
    /// ] {
    ///     assert_eq!(format!("{}", v.unwrap()), e);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// **TrapezoidalError::NotEnoughValues**: If `limits.len() < 3`.
    /// ```
    /// # use assessment::fuzzy::membership::{Trapezoidal, TrapezoidalError};
    /// let limits = vec![0.0, 0.1];
    /// assert_eq!(
    ///     Trapezoidal::new(limits.clone()),
    ///     Err(TrapezoidalError::NotEnoughValues { limits })
    /// );
    /// ```
    ///
    /// **TrapezoidalError::TooManyValues**: If `limits.len() > 4`.
    /// ```
    /// # use assessment::fuzzy::membership::{Trapezoidal, TrapezoidalError};
    /// let limits = vec![0.0, 0.1, 0.2, 0.3, 0.4];
    /// assert_eq!(
    ///     Trapezoidal::new(limits.clone()),
    ///     Err(TrapezoidalError::TooManyValues { limits })
    /// );
    /// ```
    ///
    /// **TrapezoidalError::UnorderedValues**: If `limits` are not sorted in ascending order.
    /// ```
    /// # use assessment::fuzzy::membership::{Trapezoidal, TrapezoidalError};
    /// let limits = vec![1.0, 0.1, 0.2, 0.3];
    /// assert_eq!(
    ///     Trapezoidal::new(limits.clone()),
    ///     Err(TrapezoidalError::UnorderedValues { limits })
    /// );
    /// ```
    pub fn new(limits: Vec<f32>) -> Result<Self, TrapezoidalError> {
        use TrapezoidalError::*;
        let len = limits.len();
        if len < 3 {
            Err(NotEnoughValues { limits })
        } else if len > 4 {
            Err(TooManyValues { limits })
        } else {
            for i in 0..len - 1 {
                if limits[i] > limits[i + 1] {
                    return Err(UnorderedValues { limits });
                }
            }
            Ok(Self {
                a: limits[0],
                b: limits[1],
                c: limits[len - 2],
                d: limits[len - 1],
            })
        }
    }

    /// Returns center.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::membership::Trapezoidal;
    /// for (v, e) in [
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2, 0.3]), (0.1, 0.2)),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.1, 0.2]), (0.1, 0.1)),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2]), (0.1, 0.1))
    /// ] {
    ///     assert_eq!(v.unwrap().center(), e);
    /// }
    /// ```
    ///
    pub fn center(&self) -> (f32, f32) {
        (self.b, self.c)
    }

    /// Returns coverage.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::membership::Trapezoidal;
    /// for (v, e) in [
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2, 0.3]), (0.0, 0.3)),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.1, 0.2]), (0.0, 0.2)),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2]), (0.0, 0.2))
    /// ] {
    ///     assert_eq!(v.unwrap().coverage(), e);
    /// }
    /// ```
    ///
    pub fn coverage(&self) -> (f32, f32) {
        (self.a, self.d)
    }

    /// Check if it is triangular (`b == c`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::membership::Trapezoidal;
    /// for (v, e) in [
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2, 0.3]), false),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.1, 0.2]), true),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2]), true)
    /// ] {
    ///     assert_eq!(v.unwrap().is_triangular(), e);
    /// }
    /// ```
    pub fn is_triangular(&self) -> bool {
        self.b == self.c
    }
}

/// Generates a PiecewiseLinearFunction from a trapezoidal membership
///
/// # Examples
///
/// ```
/// # use assessment::fuzzy::membership::Trapezoidal;
/// # use assessment::fuzzy::membership::piecewise::PiecewiseLinearFunction;
/// for (v, e) in [
///     (Trapezoidal::new(vec![0.0, 0.1, 0.2, 0.3]), "([0.00, 0.10] => y = 10.00·x + 0.00); ([0.10, 0.20] => y = 0.00·x + 1.00); ([0.20, 0.30] => y = -10.00·x + 3.00)"),
///     (Trapezoidal::new(vec![0.0, 0.1, 0.1, 0.2]), "([0.00, 0.10] => y = 10.00·x + 0.00); ([0.10, 0.20] => y = -10.00·x + 2.00)"),
///     (Trapezoidal::new(vec![0.0, 0.1, 0.2]), "([0.00, 0.10] => y = 10.00·x + 0.00); ([0.10, 0.20] => y = -10.00·x + 2.00)")
/// ] {
///     assert_eq!(format!("{}", PiecewiseLinearFunction::from(&v.unwrap())), e);
/// }
/// ```
///
impl From<&Trapezoidal> for PiecewiseLinearFunction {
    fn from(t: &Trapezoidal) -> Self {
        let mut result = PiecewiseLinearFunction::new();
        let (a, d) = t.coverage();
        let (b, c) = t.center();

        let extremes = |f_0, f_1, plf: &mut PiecewiseLinearFunction| {
            if f_0 != f_1 {
                let slope = 1.0 / (f_1 - f_0);
                let intercept = -1.0 * slope * f_0;
                plf.add(
                    if f_0 < f_1 { f_0 } else { f_1 },
                    if f_0 < f_1 { f_1 } else { f_0 },
                    LinearFunction::new(slope as f64, intercept as f64),
                )
                .unwrap();
            }
        };

        extremes(a as f64, b as f64, &mut result);
        extremes(d as f64, c as f64, &mut result);

        if b != c {
            result
                .add(b as f64, c as f64, LinearFunction::new(0.0, 1.0))
                .unwrap();
        }

        result
    }
}
