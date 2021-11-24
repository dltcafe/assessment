use super::Domain;

/// Quantitative domains
#[derive(Debug, PartialEq)]
pub struct Quantitative {
    inf: f64,
    sup: f64,
}

impl Domain for Quantitative {}

impl Quantitative {
    /// Quantitative domain constructors
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
    /// ```rust
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
    /// ```rust
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
    /// ```rust
    /// let inf = -1.3;
    /// let sup = 3.4;
    /// let valid_value_a = 0.7;
    /// let valid_value_b = -1.2;
    /// let invalid_value_a = inf - 0.1;
    /// let invalid_value_b = sup + 0.1;
    ///
    /// let domain = assessment::domain::Quantitative::new(inf, sup);
    ///
    /// assert_eq!(domain.valid_assessment(inf), true);
    /// assert_eq!(domain.valid_assessment(sup), true);
    /// assert_eq!(domain.valid_assessment(valid_value_a), true);
    /// assert_eq!(domain.valid_assessment(valid_value_b), true);
    /// assert_eq!(domain.valid_assessment(invalid_value_a), false);
    /// assert_eq!(domain.valid_assessment(invalid_value_b), false);
    /// ```
    pub fn valid_assessment(&self, value: f64) -> bool {
        value >= self.inf && value <= self.sup
    }
}
