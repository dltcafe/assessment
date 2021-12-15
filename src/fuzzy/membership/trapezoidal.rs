use crate::fuzzy::membership::piecewise::{LinearFunction, PiecewiseLinearFunction};
use crate::utilities;
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

    /// Returns centroid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::membership::Trapezoidal;
    /// for (v, e) in [
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2, 0.3]), 0.15),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.1, 0.2]), 0.1),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2]), 0.1)
    /// ] {
    ///     assert!((v.unwrap().centroid() - e).abs() < 0.01);
    /// }
    /// ```
    pub fn centroid(&self) -> f32 {
        let centroid_left = (self.a + (2. * self.b)) / 3.;
        let centroid_center = (self.b + self.c) / 2.;
        let centroid_right = ((2. * self.c) + self.d) / 3.;

        let area_left = (self.b - self.a) / 2.;
        let area_center = self.c - self.b;
        let area_right = (self.d - self.c) / 2.;
        let area_sum = area_left + area_center + area_right;

        ((centroid_left * area_left)
            + (centroid_center * area_center)
            + (centroid_right * area_right))
            / area_sum
    }

    /// Checks if the membership is symmetrical.
    ///
    /// ```
    /// # use assessment::fuzzy::membership::Trapezoidal;
    /// for (v, e) in [
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2, 0.3]), true),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.1, 0.2]), true),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2]), true),
    ///     (Trapezoidal::new(vec![0.0, 0.0, 0.1]), false),
    ///     (Trapezoidal::new(vec![0.0, 0.1, 0.2, 0.5]), false),
    /// ] {
    ///     assert_eq!(v.unwrap().is_symmetrical(), e);
    /// }
    /// ```
    pub fn is_symmetrical(&self) -> bool {
        ((self.b - self.a) - (self.d - self.c)).abs() < 0.01
    }

    /// Checks if the membership is symmetrical respect `other` in the `center` point.
    ///
    /// # Arguments
    /// * `other`: Trapezoidal to check if it is symmetrical respect `self`.
    /// * `center`: Center point.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::membership::Trapezoidal;
    /// for (t, o, c, e) in [
    ///     (vec![0.0, 0.1, 0.2, 0.3], vec![0.1, 0.2, 0.3, 0.4], 0.5, false),
    ///     (vec![0.0, 0.1, 0.2, 0.3], vec![0.7, 0.8, 0.9, 1.0], 0.5, true),
    ///     (vec![0.0, 0.1, 0.2, 0.3], vec![0.7, 0.8, 0.9, 1.0], 0.4, false),
    ///     (vec![0.0, 0.1, 0.2, 0.3], vec![0.5, 0.6, 0.7, 0.8], 0.4, true),
    ///     (vec![0.0, 0.1, 0.2], vec![0.8, 0.9, 1.0], 0.5, true),
    ///     (vec![0.0, 0.0, 0.1], vec![0.9, 1.0, 1.0], 0.5, true),
    ///     (vec![0.0, 0.5, 1.0], vec![0.0, 0.5, 1.0], 0.5, true),
    /// ] {
    ///     assert_eq!(Trapezoidal::new(t).unwrap().is_symmetrical_respect_center(&Trapezoidal::new(o).unwrap(), c), e);
    /// }
    /// ```
    pub fn is_symmetrical_respect_center(&self, other: &Trapezoidal, center: f32) -> bool {
        let r = 2. * center;
        utilities::math::approx_equal_f32(r - self.d, other.a, 5)
            && utilities::math::approx_equal_f32(r - self.c, other.b, 5)
            && utilities::math::approx_equal_f32(r - self.b, other.c, 5)
            && utilities::math::approx_equal_f32(r - self.a, other.d, 5)
    }

    /// Returns membership value in point `x`.
    ///
    /// # Arguments
    /// * `x`: Point in which check membership value.
    ///
    /// ```
    /// # use assessment::fuzzy::membership::Trapezoidal;
    /// let t = Trapezoidal::new(vec![0.0, 0.1, 0.2, 0.5]).unwrap();
    /// for (v, e) in [
    ///     (-0.1, 0.0),
    ///     (0.0, 0.0),
    ///     (0.05, 0.5),
    ///     (0.1, 1.0),
    ///     (0.15, 1.0),
    ///     (0.2, 1.0),
    ///     (0.25, 5./6.),
    ///     (0.5, 0.0),
    ///     (0.6, 0.0),
    /// ] {
    ///     assert!((t.membership_value(v) - e).abs() < 0.01);
    /// }
    /// ```
    pub fn membership_value(&self, x: f32) -> f32 {
        if x <= self.a || x >= self.d {
            0.
        } else if x >= self.b && x <= self.c {
            1.
        } else if x < self.b {
            (x - self.a) / (self.b - self.a)
        } else {
            (x - self.d) / (self.c - self.d)
        }
    }

    /// Returns maxmin value in interval `[max-min]`.
    ///
    /// # Arguments
    /// * `min`: Interval min value.
    /// * `max`: Interval max value.
    ///
    /// ```
    /// # use assessment::fuzzy::membership::Trapezoidal;
    /// let t = Trapezoidal::new(vec![0.0, 0.1, 0.2, 0.5]).unwrap();
    /// for (min, max, expected) in [
    ///     (0.0, 0.0, 0.0),
    ///     (0.0, 0.5, 1.0),
    ///     (0.3, 0.8, 0.67),
    ///     (0.4, 0.4, 0.33),
    /// ] {
    ///     assert!((t.max_min(min, max) - expected).abs() < 0.01, "Value {:.2} vs. Expected {:.2}", t.max_min(min, max), expected);
    /// }
    /// ```
    pub fn max_min(&self, min: f32, max: f32) -> f32 {
        if max >= self.b && min <= self.c {
            1.0
        } else if max < self.b {
            self.membership_value(max)
        } else {
            self.membership_value(min)
        }
    }
}

/// Generates a PiecewiseLinearFunction from a trapezoidal membership.
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
