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
        match &self {
            TrapezoidalError::NotEnoughValues { limits } => {
                write!(
                    f,
                    "Trapezoidal membership function needs at least 3 values, you provided {}.",
                    limits.len()
                )
            }
            TrapezoidalError::TooManyValues { limits } => {
                write!(
                    f,
                    "Trapezoidal membership function needs at most 4 values, you provided {}.",
                    limits.len()
                )
            }
            TrapezoidalError::UnorderedValues { limits: _ } => {
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
        let len = limits.len();
        if len < 3 {
            Err(TrapezoidalError::NotEnoughValues { limits })
        } else if len > 4 {
            Err(TrapezoidalError::TooManyValues { limits })
        } else {
            for i in 0..len - 1 {
                if limits[i] > limits[i + 1] {
                    return Err(TrapezoidalError::UnorderedValues { limits });
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
