use crate::domain::{Qualitative, QuantitativeLimit};
use crate::fuzzy::membership::Trapezoidal;
use crate::valuation::{Numeric, Unified, UnifiedError};

impl<'domain, T: QuantitativeLimit + Into<f64>> Numeric<'domain, T> {
    /// Unification of a Numeric valuation in a given domain.
    ///
    /// # Arguments
    /// * `domain`: Domain in which perform the unification.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_symmetric_domain;
    /// # use assessment::valuation::{Numeric, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(0, 7).unwrap();
    /// let unification_domain = qualitative_symmetric_domain!["a", "b", "c", "d", "e"].unwrap();
    ///
    /// let valuation = Numeric::new(&domain, 5).unwrap();
    /// let unified = valuation.unification(&unification_domain).unwrap();
    /// let measures = unified.measures();
    /// let expected_measures =  vec![0.0, 0.0, 0.14, 0.86, 0.0];
    /// for i in 0..(expected_measures.len()) {
    ///     assert!(
    ///         utilities::math::approx_equal_f32(
    ///             measures[i],
    ///             expected_measures[i],
    ///             2
    ///         ),
    ///         "({}) Value {:.2} vs. Expected {:.2}",
    ///         i,
    ///         measures[i],
    ///         expected_measures[i]
    ///     );
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// **UnifiedError::NonBLTSDomain**: If `domain` is a Non-BLTS domain.
    ///
    /// ```
    /// # use assessment::qualitative_symmetric_domain;
    /// # use assessment::valuation::{Numeric, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(0, 7).unwrap();
    /// let unification_domain = qualitative_symmetric_domain!["a", "b", "c", "d"].unwrap();
    ///
    /// let valuation = Numeric::new(&domain, 5).unwrap();
    /// assert_eq!(
    ///     valuation.unification(&unification_domain),
    ///     Err(UnifiedError::NonBLTSDomain { domain: &unification_domain })
    /// );
    /// ```
    ///
    pub fn unification(
        &self,
        domain: &'domain Qualitative<Trapezoidal>,
    ) -> Result<Unified, UnifiedError> {
        let value = self.normalize().value() as f32;
        let measures = (0..domain.cardinality())
            .map(|i| {
                domain
                    .get_label_by_index(i)
                    .unwrap()
                    .membership()
                    .membership_value(value)
            })
            .collect::<Vec<f32>>();
        Unified::new(domain, measures)
    }
}
