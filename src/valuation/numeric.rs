use crate::domain::Quantitative;
use crate::Valuation;

/// Numeric valuations
#[derive(Debug)]
pub struct Numeric<'domain> {
    domain: &'domain Quantitative,
    // TODO This will change to an Optional<EnumX> or something similar
    value: u32,
}

impl<'domain> Numeric<'domain> {
    /// Creates a new valuation
    ///
    /// # Examples
    ///
    /// ```rust
    /// let min = 1.0;
    /// let max = 5.7;
    ///
    /// let value_a = 4;
    /// let value_b = 3;
    ///
    /// let domain = assessment::domain::Quantitative::new(min, max);
    /// dbg!(assessment::valuation::Numeric::new(&domain, value_a));
    /// dbg!(assessment::valuation::Numeric::new(&domain, value_b));
    /// ```
    ///
    /// # Panics
    ///
    /// If `value` is not a valid assessment in `domain`
    pub fn new(domain: &'domain Quantitative, value: u32) -> Self {
        if !domain.valid_assessment(value as f64) {
            panic!(
                "Value {} cannot be used as a valuation in domain {:?}",
                value, domain
            );
        }

        Self { domain, value }
    }

    /// Returns valuation value
    ///
    /// # Examples
    ///
    /// ```rust
    /// let min = 1.0;
    /// let max = 5.7;
    ///
    /// let value = 4;
    ///
    /// let domain = assessment::domain::Quantitative::new(min, max);
    /// let valuation = assessment::valuation::Numeric::new(&domain, value);
    ///
    /// assert_eq!(valuation.value(), value);
    /// ```
    pub fn value(&self) -> u32 {
        self.value
    }

    /// Returns valuation domain
    ///
    /// # Examples
    ///
    /// ```rust
    /// let min = 1.0;
    /// let max = 5.7;
    ///
    /// let value = 4;
    ///
    /// let domain = assessment::domain::Quantitative::new(min, max);
    /// let valuation = assessment::valuation::Numeric::new(&domain, value);
    /// assert_eq!(*valuation.domain(), domain);
    /// ```
    pub fn domain(&self) -> &'domain Quantitative {
        self.domain
    }
}

impl<'domain> Valuation for Numeric<'domain> {}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn valuation_new_if_value_is_outside_domain_limits() {
        let min = 1.0;
        let max = 5.7;

        let value = min - 1.0;

        let domain = crate::domain::Quantitative::new(min, max);
        crate::valuation::Numeric::new(&domain, value as u32);
    }
}
