use super::Domain;

/// Quantitative domains
#[derive(Debug, PartialEq)]
pub struct Quantitative {
    inf: f64,
    sup: f64,
}

// // //
// Traits implementations
//

impl Domain for Quantitative {}

// // //
// Implementation
//

impl Quantitative {
    /// Quantitative domain constructor
    ///
    /// # Params
    /// - `inf`: Domain inferior limit.
    /// - `sup`: Domain superior limit.
    ///
    /// # Examples
    ///
    /// ```
    /// assessment::domain::Quantitative::new(-1.3, 3.4);
    /// ```
    ///
    /// ```
    /// assessment::domain::Quantitative::new(-1.3, -1.3);
    /// ```
    ///
    /// # Panics
    ///
    /// If `inf` > `sup`
    ///
    /// ```should_panic
    /// assessment::domain::Quantitative::new(10.0, 5.0);
    /// ```
    pub fn new(inf: f64, sup: f64) -> Self {
        if inf > sup {
            panic!("min ({}) > Max ({})", inf, sup)
        }

        Self { inf, sup }
    }

    /// Returns domain inferior limit
    ///
    /// # Examples
    ///
    /// ```
    /// let inf = -1.3;
    /// let domain = assessment::domain::Quantitative::new(inf, 0.0);
    /// assert_eq!(domain.inf(), inf);
    /// ```
    pub fn inf(&self) -> f64 {
        self.inf
    }

    /// Returns domain superior limit
    ///
    /// # Examples
    ///
    /// ```
    /// let sup = 3.4;
    /// let domain = assessment::domain::Quantitative::new(0.0, sup);
    /// assert_eq!(domain.sup(), sup);
    /// ```
    pub fn sup(&self) -> f64 {
        self.sup
    }

    /// Check if a given value is a valid assessment in the current domain.
    ///
    /// # Params
    /// - `value`: Value to be checked.
    ///
    /// # Examples
    ///
    /// ```
    /// let inf = -1.3;
    /// let sup = inf + 1.0;
    ///
    /// let domain = assessment::domain::Quantitative::new(inf, sup);
    ///
    /// for (v, e) in [
    ///     (inf, true),
    ///     (sup, true),
    ///     (inf + 0.1, true),
    ///     (sup - 0.1, true),
    ///     (inf - 0.1, false),
    ///     (sup + 0.1, false),
    /// ] {
    ///     assert_eq!(domain.valid_assessment(v), e);
    /// }
    /// ```
    pub fn valid_assessment(&self, value: f64) -> bool {
        value >= self.inf && value <= self.sup
    }
}
