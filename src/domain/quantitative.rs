use super::Domain;

/// Quantitative domains
#[derive(Debug, PartialEq)]
pub struct Quantitative {
    min: f64,
    max: f64,
}

impl Domain for Quantitative {}

impl Quantitative {
    /// Quantitative domain constructors
    ///
    /// # Examples
    ///
    /// ```rust
    /// let min = -1.3;
    /// let max = 3.4;
    ///
    /// let domain = assessment::domain::Quantitative::new(min, max);
    /// dbg!(domain);
    /// ```
    ///
    /// ```rust
    /// let min = -1.3;
    /// let max = min;
    ///
    /// let domain = assessment::domain::Quantitative::new(min, max);
    /// dbg!(domain);
    /// ```
    ///
    /// # Panics
    ///
    /// If `min` > `max`
    pub fn new(min: f64, max: f64) -> Self {
        if min > max {
            panic!("min ({}) > Max ({})", min, max)
        }

        Self { min, max }
    }

    /// Returns domain inferior limit
    ///
    /// # Examples
    ///
    /// ```rust
    /// let min = -1.3;
    /// let max = 3.4;
    ///
    /// let domain = assessment::domain::Quantitative::new(min, max);
    ///
    /// assert_eq!(domain.min(), min);
    /// ```
    pub fn min(&self) -> f64 {
        self.min
    }

    /// Returns domain superior limit
    ///
    /// # Examples
    ///
    /// ```rust
    /// let min = -1.3;
    /// let max = 3.4;
    ///
    /// let domain = assessment::domain::Quantitative::new(min, max);
    ///
    /// assert_eq!(domain.max(), max);
    /// ```
    pub fn max(&self) -> f64 {
        self.max
    }

    /// Check if a given value is a valid assessment in the current domain.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let min = -1.3;
    /// let max = 3.4;
    /// let valid_value_a = 0.7;
    /// let valid_value_b = -1.2;
    /// let invalid_value_a = min - 0.1;
    /// let invalid_value_b = max + 0.1;
    ///
    /// let domain = assessment::domain::Quantitative::new(min, max);
    ///
    /// assert_eq!(domain.valid_assessment(min), true);
    /// assert_eq!(domain.valid_assessment(max), true);
    /// assert_eq!(domain.valid_assessment(valid_value_a), true);
    /// assert_eq!(domain.valid_assessment(valid_value_b), true);
    /// assert_eq!(domain.valid_assessment(invalid_value_a), false);
    /// assert_eq!(domain.valid_assessment(invalid_value_b), false);
    /// ```
    pub fn valid_assessment(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn quantitative_new_if_min_is_greater_than_max() {
        let min = 1.0;
        let max = min - 1.0;

        crate::domain::Quantitative::new(min, max);
    }
}
