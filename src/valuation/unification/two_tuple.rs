use crate::domain::Qualitative;
use crate::fuzzy::membership::Trapezoidal;
use crate::fuzzy::LabelMembership;
use crate::utilities;
use crate::valuation::{Single, TwoTuple, TwoTupleError, Unified, UnifiedError};
use std::fmt::Display;

impl<'domain> TwoTuple<'domain, Trapezoidal> {
    /// Unification of a valuation in a new domain.
    ///
    /// # Arguments
    /// * `domain`: Domain in which perform the unification.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_symmetric_domain;
    /// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
    /// let unification_domain = qualitative_symmetric_domain!["a", "b", "c", "d", "e"].unwrap();
    ///
    /// let valuation = TwoTuple::new_by_label_index(&domain, 1, 0.3).unwrap();
    /// let unified = valuation.unification_in_domain(&unification_domain).unwrap();
    /// let measures = unified.measures();
    /// let expected_measures =  vec![0.0, 0.0, 0.4, 0.6, 0.0];
    /// for i in 0..(expected_measures.len()) {
    ///     assert!(
    ///         utilities::math::approx_equal_f32(
    ///             measures[i],
    ///             expected_measures[i],
    ///             5
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
    /// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
    /// let unification_domain = qualitative_symmetric_domain!["a", "b", "c", "d"].unwrap();
    ///
    /// let valuation = TwoTuple::new_by_label_index(&domain, 1, 0.3).unwrap();
    /// assert_eq!(
    ///     valuation.unification_in_domain(&unification_domain),
    ///     Err(UnifiedError::NonBLTSDomain { domain: &unification_domain })
    /// );
    /// ```
    ///
    pub fn unification_in_domain(
        &self,
        domain: &'domain Qualitative<Trapezoidal>,
    ) -> Result<Unified, UnifiedError<'domain>> {
        let beta = (self.inverse_delta() * (domain.cardinality() - 1) as f32)
            / (self.domain().cardinality() - 1) as f32;
        let index = beta.round() as usize;
        let alpha = utilities::math::round_f32(beta - index as f32, 5);

        let mut measures: Vec<f32> = vec![0.; domain.cardinality()];
        measures[index] = 1. - alpha.abs();
        if alpha != 0. {
            measures[if alpha > 0. { index + 1 } else { index - 1 }] = alpha.abs()
        }
        Unified::new(&domain, measures)
    }

    /// Transform into a TwoTuple valuation in a different domain.
    ///
    /// # Arguments
    /// * `domain`: Domain to be used.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_symmetric_domain;
    /// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
    /// let unification_domain = qualitative_symmetric_domain!["a", "b", "c", "d", "e"].unwrap();
    ///
    /// let valuation = TwoTuple::new_by_label_index(&domain, 1, 0.3).unwrap();
    /// let transformed = valuation.transform_in_domain(&unification_domain).unwrap();
    /// let expected = TwoTuple::new_by_label_index(&unification_domain, 3, -0.4).unwrap();
    /// assert_eq!(transformed, expected);
    /// ```
    ///
    /// # Errors
    ///
    /// **UnifiedError::NonBLTSDomain**: If `domain` is a Non-BLTS domain.
    ///
    /// ```
    /// # use assessment::qualitative_symmetric_domain;
    /// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
    /// let unification_domain = qualitative_symmetric_domain!["a", "b", "c", "d"].unwrap();
    ///
    /// let valuation = TwoTuple::new_by_label_index(&domain, 1, 0.3).unwrap();
    /// assert_eq!(
    ///     valuation.transform_in_domain(&unification_domain),
    ///     Err(UnifiedError::NonBLTSDomain { domain: &unification_domain })
    /// );
    /// ```
    ///
    pub fn transform_in_domain(
        &self,
        domain: &'domain Qualitative<Trapezoidal>,
    ) -> Result<Self, UnifiedError<'domain>> {
        if !domain.is_blts() {
            Err(UnifiedError::NonBLTSDomain { domain: &domain })
        } else {
            Ok(TwoTuple::delta(
                &domain,
                (self.inverse_delta() * (domain.cardinality() - 1) as f32)
                    / (self.domain().cardinality() - 1) as f32,
            )
            .unwrap())
        }
    }
}

/// Generates a Unified valuation from a &TwoTuple valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_symmetric_domain;
/// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
/// # use assessment::utilities;
/// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
///
/// let valuation = TwoTuple::new_by_label_index(&domain, 1, 0.3).unwrap();
/// let unified = Unified::try_from(&valuation).unwrap();
/// assert_eq!(*unified.measures(), vec![0.0, 0.7, 0.3]);
/// assert!(utilities::math::approx_equal_f32(unified.chi(), valuation.inverse_delta(), 5));
/// ```
///
/// # Errors
///
/// **UnifiedError::NonBLTSDomain**: If valuation domain is a Non-BLTS domain.
///
/// ```
/// # use assessment::qualitative_symmetric_domain;
/// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
/// # use assessment::utilities;
/// let domain = qualitative_symmetric_domain!["a", "b"].unwrap();
///
/// let valuation = TwoTuple::new_by_label_index(&domain, 0, 0.3).unwrap();
/// assert_eq!(
///     Unified::try_from(&valuation),
///     Err(UnifiedError::NonBLTSDomain { domain: &domain })
/// );
/// ```
///
impl<'domain> TryFrom<&TwoTuple<'domain, Trapezoidal>> for Unified<'domain> {
    type Error = UnifiedError<'domain>;

    fn try_from(value: &TwoTuple<'domain, Trapezoidal>) -> Result<Self, Self::Error> {
        let mut measures: Vec<f32> = vec![0.; value.domain().cardinality()];
        let index = value.index();
        let alpha = value.alpha();
        measures[index] = utilities::math::round_f32(1. - alpha.abs(), 5);
        if alpha != 0. {
            measures[if alpha > 0. { index + 1 } else { index - 1 }] = alpha.abs()
        }
        Unified::new(&value.domain(), measures)
    }
}

/// Generates a TwoTuple valuation from an &Unified valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_symmetric_domain;
/// # use assessment::valuation::{TwoTuple, Unified, UnifiedError};
/// # use assessment::utilities;
/// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
///
/// let valuation = Unified::new(&domain, vec![0.0, 0.7, 0.3]).unwrap();
/// let two_tuple = TwoTuple::try_from(&valuation).unwrap();
/// assert_eq!(*two_tuple.domain(), domain);
/// assert!(utilities::math::approx_equal_f32(two_tuple.inverse_delta(), valuation.chi(), 5));
/// ```
///
impl<'domain> TryFrom<&Unified<'domain>> for TwoTuple<'domain, Trapezoidal> {
    type Error = TwoTupleError<'domain, Trapezoidal>;

    fn try_from(value: &Unified<'domain>) -> Result<Self, Self::Error> {
        TwoTuple::delta(value.domain(), value.chi())
    }
}

/// Generates a TwoTuple valuation from an &Single valuation.
///
/// # Examples
///
/// ```
///
/// # use assessment::qualitative_symmetric_domain;
/// # use assessment::valuation::{TwoTuple, Single, TwoTupleError};
/// # use assessment::utilities;
/// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
///
/// let valuation = Single::new_by_label_index(&domain, 1).unwrap();
/// let two_tuple = TwoTuple::try_from(&valuation).unwrap();
/// assert_eq!(two_tuple.index(), 1);
/// assert_eq!(two_tuple.alpha(), 0.0);
/// ```
///
impl<'domain, T: LabelMembership + Display> TryFrom<&Single<'domain, T>> for TwoTuple<'domain, T> {
    type Error = TwoTupleError<'domain, T>;

    fn try_from(value: &Single<'domain, T>) -> Result<Self, Self::Error> {
        TwoTuple::new_by_label_index(value.domain(), value.index(), 0.)
    }
}

/// Generates a Unified valuation from a TwoTuple valuation.
///
/// Wrapper of Unified::try_from(&TwoTuple).
///
impl<'domain> TryFrom<TwoTuple<'domain, Trapezoidal>> for Unified<'domain> {
    type Error = UnifiedError<'domain>;

    fn try_from(value: TwoTuple<'domain, Trapezoidal>) -> Result<Self, Self::Error> {
        Unified::try_from(&value)
    }
}

/// Generates a TwoTuple valuation from an Unified valuation.
///
/// Wrapper of TwoTuple::try_from(&Unified).
///
impl<'domain> TryFrom<Unified<'domain>> for TwoTuple<'domain, Trapezoidal> {
    type Error = TwoTupleError<'domain, Trapezoidal>;

    fn try_from(value: Unified<'domain>) -> Result<Self, Self::Error> {
        TwoTuple::try_from(&value)
    }
}

/// Generates a TwoTuple valuation from a Single valuation.
///
/// Wrapper of TwoTuple::try_from(&Single).
///
impl<'domain, T: LabelMembership + Display> TryFrom<Single<'domain, T>> for TwoTuple<'domain, T> {
    type Error = TwoTupleError<'domain, T>;

    fn try_from(value: Single<'domain, T>) -> Result<Self, Self::Error> {
        TwoTuple::try_from(&value)
    }
}
