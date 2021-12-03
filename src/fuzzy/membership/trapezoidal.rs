use super::Membership;
use std::fmt::{Display, Formatter};

/// Trapezoidal Membership Function struct
///
/// This function is defined by four points going from left to right (a, b, c, d).
///
/// [a-d] is the base of the trapezoid and [b-c] is the center.
///
/// If b=c the function is called **Triangular**.
#[derive(Debug, PartialEq, Clone)]
pub struct Trapezoidal {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

// // //
// Traits implementations
//

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

// // //
// Implementation
//

impl Trapezoidal {
    /// Trapezoidal membership function constructor
    ///
    /// # Params
    /// - `limits`: Membership function limits
    ///
    /// # Examples
    ///
    /// ```
    /// use assessment::fuzzy::membership::trapezoidal::*;
    /// let trapezoidal = Trapezoidal::new(&vec![0.0, 0.1, 0.2, 0.3]);
    /// assert_eq!(format!("{}", trapezoidal), "(0.00, 0.10, 0.20, 0.30)");
    /// ```
    ///
    /// ```
    /// use assessment::fuzzy::membership::trapezoidal::*;
    /// let trapezoidal = Trapezoidal::new(&vec![0.0, 0.1, 0.1, 0.2]);
    /// assert_eq!(format!("{}", trapezoidal), "(0.00, 0.10, 0.20)");
    /// ```
    ///
    /// ```
    /// use assessment::fuzzy::membership::trapezoidal::*;
    /// let trapezoidal = Trapezoidal::new(&vec![0.0, 0.1, 0.2]);
    /// assert_eq!(format!("{}", trapezoidal), "(0.00, 0.10, 0.20)");
    /// ```
    ///
    /// # Panics
    ///
    /// If `limits.len()` < 3
    /// ```should_panic
    /// assessment::fuzzy::membership::trapezoidal::Trapezoidal::new(&vec![0.0, 0.1]);
    /// ```
    ///
    /// If `limits.len()` > 4
    /// ```should_panic
    /// assessment::fuzzy::membership::trapezoidal::Trapezoidal::new(&vec![0.0, 0.1, 0.2, 0.3, 0.4]);
    /// ```
    ///
    /// If `limits` are not sorted in ascending order
    /// ```should_panic
    /// assessment::fuzzy::membership::trapezoidal::Trapezoidal::new(&vec![1.0, 0.1, 0.2, 0.3]);
    /// ```
    /// ```should_panic
    /// assessment::fuzzy::membership::trapezoidal::Trapezoidal::new(&vec![1.0, 1.1, 0.2, 0.3]);
    /// ```
    /// ```should_panic
    /// assessment::fuzzy::membership::trapezoidal::Trapezoidal::new(&vec![1.0, 1.1, 1.2, 0.3]);
    /// ```
    pub fn new(limits: &[f32]) -> Self {
        let len = &limits.len();

        // Force 3/4 elements
        if !(3..5).contains(len) {
            panic!(
                "Trapezoidal membership function needs 3 or 4 values, you provided {}",
                len
            );
        }

        // Force order
        for i in 0..len - 1 {
            if &limits[i] > &limits[i + 1] {
                panic!("Trapezoidal membership function needs an ordered array of values. limits[{}] = {} > {} = limits[{}])", i, &limits[i], &limits[i + 1], i + 1);
            }
        }

        if *len as u8 == 3 {
            Self {
                a: limits[0],
                b: limits[1],
                c: limits[1],
                d: limits[2],
            }
        } else {
            Self {
                a: limits[0],
                b: limits[1],
                c: limits[2],
                d: limits[3],
            }
        }
    }

    /// Check if it is triangular (b == c)
    ///
    /// # Examples
    ///
    /// ```
    /// use assessment::fuzzy::membership::trapezoidal::*;
    /// let trapezoidal = Trapezoidal::new(&vec![0.0, 0.1, 0.2, 0.3]);
    /// assert_eq!(trapezoidal.is_triangular(), false);
    /// ```
    ///
    /// ```
    /// use assessment::fuzzy::membership::trapezoidal::*;
    /// let trapezoidal = Trapezoidal::new(&vec![0.0, 0.2, 0.2, 0.3]);
    /// assert_eq!(trapezoidal.is_triangular(), true);
    /// ```
    ///
    /// ```
    /// use assessment::fuzzy::membership::trapezoidal::*;
    /// let trapezoidal = Trapezoidal::new(&vec![0.0, 0.1, 0.2]);
    /// assert_eq!(trapezoidal.is_triangular(), true);
    /// ```
    pub fn is_triangular(&self) -> bool {
        self.b == self.c
    }
}
