use crate::domain::{Qualitative, Quantitative, QuantitativeLimit};
use crate::fuzzy::membership::Trapezoidal;
use crate::valuation::{Interval, Numeric, NumericError, Unified, UnifiedError};
use std::ops::{Add, Div, Mul, Sub};

impl<'domain, T: QuantitativeLimit + Into<f64>> Numeric<'domain, T>
where
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Div<Output = T>,
{
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

    /// Transform a Numeric valuation using a different domain.
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
    /// # use assessment::valuation::{Numeric, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// # use assessment::domain::Quantitative;
    /// let domain = Quantitative::new(0, 7).unwrap();
    /// let transform_domain = Quantitative::new(0, 3).unwrap();
    ///
    /// let valuation = Numeric::new(&domain, 5).unwrap();
    /// let unified = valuation.transform_in_domain(&transform_domain);
    /// assert_eq!(unified.value(), 2);
    /// ```
    ///
    pub fn transform_in_domain(&self, domain: &'domain Quantitative<T>) -> Numeric<T> {
        Numeric::new(
            domain,
            crate::utilities::math::transform_range(
                self.value(),
                self.domain().inf(),
                self.domain().sup(),
                domain.inf(),
                domain.sup(),
            ),
        )
        .unwrap()
    }
}

/// Generates a Numeric<f32> valuation from an &Interval<f32> valuation.
///
/// # Examples
///
/// ```
/// # use assessment::domain::Quantitative;
/// # use assessment::utilities;
/// # use assessment::valuation::{Numeric, Interval};
/// let domain = Quantitative::new(0.5_f32, 1.0_f32).unwrap();
/// let interval = Interval::new(&domain, 0.6, 0.8).unwrap();
/// let numeric = Numeric::try_from(&interval).unwrap();
/// let expected = 0.7;
/// assert!((numeric.value() - expected).abs() < 0.01);
/// ```
///
impl<'domain> TryFrom<&Interval<'domain, f32>> for Numeric<'domain, f32> {
    type Error = NumericError<f32>;

    fn try_from(value: &Interval<'domain, f32>) -> Result<Self, Self::Error> {
        Numeric::new(value.domain(), value.resume())
    }
}

/// Generates a Numeric<f64> valuation from an &Interval<f64> valuation.
///
/// # Examples
///
/// ```
/// # use assessment::domain::Quantitative;
/// # use assessment::utilities;
/// # use assessment::valuation::{Numeric, Interval};
/// let domain = Quantitative::new(0.5_f64, 1.0_f64).unwrap();
/// let interval = Interval::new(&domain, 0.6, 0.8).unwrap();
/// let numeric = Numeric::try_from(&interval).unwrap();
/// let expected = 0.7;
/// assert!((numeric.value() - expected).abs() < 0.01);
/// ```
///
impl<'domain> TryFrom<&Interval<'domain, f64>> for Numeric<'domain, f64> {
    type Error = NumericError<f64>;

    fn try_from(value: &Interval<'domain, f64>) -> Result<Self, Self::Error> {
        Numeric::new(value.domain(), value.resume())
    }
}

/// Generates a Numeric<i32> valuation from an &Interval<i32> valuation.
///
/// # Examples
///
/// ```
/// # use assessment::domain::Quantitative;
/// # use assessment::utilities;
/// # use assessment::valuation::{Numeric, Interval};
/// let domain = Quantitative::new(5, 10).unwrap();
/// let interval = Interval::new(&domain, 6, 8).unwrap();
/// let numeric = Numeric::try_from(&interval).unwrap();
/// let expected = 7;
/// assert_eq!(numeric.value(), expected);
/// ```
///
impl<'domain> TryFrom<&Interval<'domain, i32>> for Numeric<'domain, i32> {
    type Error = NumericError<i32>;

    fn try_from(value: &Interval<'domain, i32>) -> Result<Self, Self::Error> {
        Numeric::new(value.domain(), value.resume())
    }
}

/// Generates a Numeric<f32> valuation from an Interval<f32> valuation.
///
/// Wrapper of Numeric::try_from(&Interval<f32>).
///
impl<'domain> TryFrom<Interval<'domain, f32>> for Numeric<'domain, f32> {
    type Error = NumericError<f32>;

    fn try_from(value: Interval<'domain, f32>) -> Result<Self, Self::Error> {
        Numeric::try_from(&value)
    }
}

/// Generates a Numeric<f64> valuation from an Interval<f64> valuation.
///
/// Wrapper of Numeric::try_from(&Interval<f64>).
///
impl<'domain> TryFrom<Interval<'domain, f64>> for Numeric<'domain, f64> {
    type Error = NumericError<f64>;

    fn try_from(value: Interval<'domain, f64>) -> Result<Self, Self::Error> {
        Numeric::try_from(&value)
    }
}

/// Generates a Numeric<i32> valuation from an Interval<i32> valuation.
///
/// Wrapper of Numeric::try_from(&Interval<i32>).
///
/// ```
/// # use assessment::domain::Quantitative;
/// # use assessment::utilities;
/// # use assessment::valuation::{Numeric, Interval};
/// let domain = Quantitative::new(5, 10).unwrap();
/// let interval = Interval::new(&domain, 6, 8).unwrap();
/// let numeric = Numeric::try_from(&interval).unwrap();
/// let expected = 7;
/// assert_eq!(numeric.value(), expected);
/// ```
///
impl<'domain> TryFrom<Interval<'domain, i32>> for Numeric<'domain, i32> {
    type Error = NumericError<i32>;

    fn try_from(value: Interval<'domain, i32>) -> Result<Self, Self::Error> {
        Numeric::try_from(&value)
    }
}
