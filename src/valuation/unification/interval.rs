use crate::domain::{Qualitative, Quantitative, QuantitativeLimit};
use crate::fuzzy::membership::Trapezoidal;
use crate::valuation::{Interval, IntervalError, Numeric, Unified, UnifiedError};
use std::ops::{Add, Div, Mul, Sub};

impl<'domain, T: QuantitativeLimit + Into<f64>> Interval<'domain, T>
where
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Div<Output = T>,
{
    /// Unification of a Interval valuation in a given domain.
    ///
    /// # Arguments
    /// * `domain`: Domain in which perform the unification.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_symmetric_domain;
    /// # use assessment::valuation::{Interval, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(0, 7).unwrap();
    /// let unification_domain = qualitative_symmetric_domain!["a", "b", "c", "d", "e"].unwrap();
    ///
    /// let valuation = Interval::new(&domain, 5, 6).unwrap();
    /// let unified = valuation.unification(&unification_domain).unwrap();
    /// let measures = unified.measures();
    /// let expected_measures =  vec![0.0, 0.0, 0.14, 1.0, 0.43];
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
    /// # use assessment::valuation::{Interval, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(0, 7).unwrap();
    /// let unification_domain = qualitative_symmetric_domain!["a", "b", "c", "d"].unwrap();
    ///
    /// let valuation = Interval::new(&domain, 5, 6).unwrap();
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
        let (min, max) = self.normalize().value();
        let min_f32 = min as f32;
        let max_f32 = max as f32;
        let measures = (0..domain.cardinality())
            .map(|i| {
                domain
                    .get_label_by_index(i)
                    .unwrap()
                    .membership()
                    .max_min(min_f32, max_f32)
            })
            .collect::<Vec<f32>>();
        Unified::new(domain, measures)
    }

    /// Transform a Interval valuation using a different domain.
    ///
    /// Note that domain type should be equal to valuation type.
    ///
    /// # Arguments
    ///
    /// * `domain`: Domain to be used.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_symmetric_domain;
    /// # use assessment::valuation::Interval;
    /// # use assessment::utilities;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(0, 7).unwrap();
    /// let transform_domain = Quantitative::new(0, 3).unwrap();
    ///
    /// let valuation = Interval::new(&domain, 5, 6).unwrap();
    /// let unified = valuation.transform_in_domain(&transform_domain);
    /// assert_eq!(unified.value(), (2, 2));
    /// ```
    ///
    pub fn transform_in_domain(&self, domain: &'domain Quantitative<T>) -> Interval<T> {
        let (old_min, old_max) = self.value();
        Interval::new(
            domain,
            crate::utilities::math::transform_range(
                old_min,
                self.domain().inf(),
                self.domain().sup(),
                domain.inf(),
                domain.sup(),
            ),
            crate::utilities::math::transform_range(
                old_max,
                self.domain().inf(),
                self.domain().sup(),
                domain.inf(),
                domain.sup(),
            ),
        )
        .unwrap()
    }
}

/// Generates an Interval valuation from a &Numeric valuation.
///
/// # Examples
///
/// ```
/// # use assessment::domain::Quantitative;
/// # use assessment::utilities;
/// # use assessment::valuation::{Numeric, Interval};
/// let domain = Quantitative::new(5, 10).unwrap();
/// let numeric = Numeric::new(&domain, 6).unwrap();
/// let interval = Interval::try_from(&numeric).unwrap();
/// let expected = (6, 6);
/// assert_eq!(interval.value(), expected);
/// ```
///
impl<'domain, T: QuantitativeLimit + Into<f64> + Add<Output = T> + Sub<Output = T>>
    TryFrom<&Numeric<'domain, T>> for Interval<'domain, T>
{
    type Error = IntervalError<T>;

    fn try_from(value: &Numeric<'domain, T>) -> Result<Self, Self::Error> {
        Interval::new(value.domain(), value.value(), value.value())
    }
}

/// Generates an Interval valuation from a Numeric valuation.
///
/// Wrapper of Interval::try_from(&Numeric).
///
impl<'domain, T: QuantitativeLimit + Into<f64> + Add<Output = T> + Sub<Output = T>>
    TryFrom<Numeric<'domain, T>> for Interval<'domain, T>
{
    type Error = IntervalError<T>;

    fn try_from(value: Numeric<'domain, T>) -> Result<Self, Self::Error> {
        Interval::try_from(&value)
    }
}
