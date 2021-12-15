use crate::domain::Qualitative;
use crate::fuzzy::membership::Trapezoidal;
use crate::fuzzy::LabelMembership;
use crate::valuation::{Single, SingleError, TwoTuple, Unified, UnifiedError};
use std::fmt::Display;

impl<'domain> Single<'domain, Trapezoidal> {
    /// Unification of a Single valuation in a new domain.
    ///
    /// # Arguments
    /// * `domain`: Domain in which perform the unification.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_symmetric_domain;
    /// # use assessment::valuation::{Single, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
    /// let unification_domain = qualitative_symmetric_domain!["a", "b", "c", "d", "e"].unwrap();
    ///
    /// let valuation = Single::new_by_label_index(&domain, 1).unwrap();
    /// let unified = valuation.unification_in_domain(&unification_domain).unwrap();
    /// let measures = unified.measures();
    /// let expected_measures =  vec![0.0, 0.0, 1.0, 0.0, 0.0];
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
    /// # use assessment::valuation::{Single, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
    /// let unification_domain = qualitative_symmetric_domain!["a", "b", "c", "d"].unwrap();
    ///
    /// let valuation = Single::new_by_label_index(&domain, 1).unwrap();
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
        let mut measures: Vec<f32> = vec![0.; domain.cardinality()];
        measures[self.index() * (domain.cardinality() - 1) / (self.domain().cardinality() - 1)] =
            1.;
        Unified::new(domain, measures)
    }

    /// Transform into a Single valuation in a different domain.
    ///
    /// # Arguments
    /// * `domain`: Domain to be used.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_symmetric_domain;
    /// # use assessment::valuation::{Single, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
    /// let unification_domain = qualitative_symmetric_domain!["a", "b", "c", "d", "e"].unwrap();
    ///
    /// let valuation = Single::new_by_label_index(&domain, 1).unwrap();
    /// let transformed = valuation.transform_in_domain(&unification_domain).unwrap();
    /// let expected = Single::new_by_label_index(&unification_domain, 2).unwrap();
    /// assert_eq!(transformed, expected);
    /// ```
    ///
    /// # Errors
    ///
    /// **UnifiedError::NonBLTSDomain**: If `domain` is a Non-BLTS domain.
    ///
    /// ```
    /// # use assessment::qualitative_symmetric_domain;
    /// # use assessment::valuation::{Single, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
    /// let unification_domain = qualitative_symmetric_domain!["a", "b", "c", "d"].unwrap();
    ///
    /// let valuation = Single::new_by_label_index(&domain, 1).unwrap();
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
            Err(UnifiedError::NonBLTSDomain { domain })
        } else {
            Ok(Single::new_by_label_index(
                domain,
                self.index() * (domain.cardinality() - 1) / (self.domain().cardinality() - 1),
            )
            .unwrap())
        }
    }
}

/// Generates a Unified valuation from a &Linguistic valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_symmetric_domain;
/// # use assessment::valuation::{Single, Unified, UnifiedError};
/// # use assessment::utilities;
/// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();///
/// let valuation = Single::new_by_label_index(&domain, 1).unwrap();
/// let unified = Unified::try_from(&valuation).unwrap();
/// assert_eq!(*unified.measures(), vec![0.0, 1.0, 0.0]);
/// ```
///
/// # Errors
///
/// **UnifiedError::NonBLTSDomain**: If valuation domain is a Non-BLTS domain.
///
/// ```
/// # use assessment::qualitative_symmetric_domain;
/// # use assessment::valuation::{Single, Unified, UnifiedError};
/// # use assessment::utilities;
/// let domain = qualitative_symmetric_domain!["a", "b"].unwrap();
///
/// let valuation = Single::new_by_label_index(&domain, 1).unwrap();
/// assert_eq!(
///     Unified::try_from(&valuation),
///     Err(UnifiedError::NonBLTSDomain { domain: &domain })
/// );
/// ```
///
impl<'domain> TryFrom<&Single<'domain, Trapezoidal>> for Unified<'domain> {
    type Error = UnifiedError<'domain>;

    fn try_from(value: &Single<'domain, Trapezoidal>) -> Result<Self, Self::Error> {
        let mut measures: Vec<f32> = vec![0.; value.domain().cardinality()];
        measures[value.index()] = 1.;
        Unified::new(value.domain(), measures)
    }
}

/// Generates a Single valuation from an &Unified valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_symmetric_domain;
/// # use assessment::utilities;
/// # use assessment::valuation::{Single, Unified, UnifiedError};
/// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
///
/// let valuation = Unified::new(&domain, vec![0.0, 0.7, 0.3]).unwrap();
/// let single = Single::try_from(&valuation).unwrap();
/// assert_eq!(single.index(), 1);
/// ```
///
impl<'domain> TryFrom<&Unified<'domain>> for Single<'domain, Trapezoidal> {
    type Error = SingleError<'domain, Trapezoidal>;

    fn try_from(value: &Unified<'domain>) -> Result<Self, Self::Error> {
        Single::new_by_label_index(value.domain(), value.chi().round() as usize)
    }
}

/// Generates a Single valuation from an &TwoTuple valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_symmetric_domain;
/// # use assessment::utilities;
/// # use assessment::valuation::{Single, TwoTuple, SingleError};
/// let domain = qualitative_symmetric_domain!["a", "b", "c"].unwrap();
///
/// let valuation = TwoTuple::new_by_label_index(&domain, 1, 0.3).unwrap();
/// let single = Single::try_from(&valuation).unwrap();
/// assert_eq!(single.index(), 1);
/// ```
///
impl<'domain, T: LabelMembership + Display> TryFrom<&TwoTuple<'domain, T>> for Single<'domain, T> {
    type Error = SingleError<'domain, T>;

    fn try_from(value: &TwoTuple<'domain, T>) -> Result<Self, Self::Error> {
        Single::new_by_label_index(value.domain(), value.index())
    }
}

/// Generates a Unified valuation from a Linguistic valuation.
///
/// Wrapper of Unified::try_from(&Linguistic).
///
impl<'domain> TryFrom<Single<'domain, Trapezoidal>> for Unified<'domain> {
    type Error = UnifiedError<'domain>;

    fn try_from(value: Single<'domain, Trapezoidal>) -> Result<Self, Self::Error> {
        Unified::try_from(&value)
    }
}

/// Generates a Single valuation from an Unified valuation.
///
/// Wrapper of Single::try_from(&Unified).
///
impl<'domain> TryFrom<Unified<'domain>> for Single<'domain, Trapezoidal> {
    type Error = SingleError<'domain, Trapezoidal>;

    fn try_from(value: Unified<'domain>) -> Result<Self, Self::Error> {
        Single::try_from(&value)
    }
}

/// Generates a Single valuation from a TwoTuple valuation.
///
/// Wrapper of Single::try_from(&TwoTuple).
///
impl<'domain, T: LabelMembership + Display> TryFrom<TwoTuple<'domain, T>> for Single<'domain, T> {
    type Error = SingleError<'domain, T>;

    fn try_from(value: TwoTuple<'domain, T>) -> Result<Self, Self::Error> {
        Single::try_from(&value)
    }
}
