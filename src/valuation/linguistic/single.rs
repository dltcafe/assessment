use crate::domain::Qualitative;
use crate::fuzzy::membership::Trapezoidal;
use crate::fuzzy::{Label, LabelMembership};
use crate::valuation::{Linguistic, TwoTuple, Unified, UnifiedError};
use crate::Valuation;
use std::fmt::{Display, Formatter};

/// Single linguistic valuations
#[derive(Debug, PartialEq)]
pub struct Single<'domain, T: LabelMembership> {
    domain: &'domain Qualitative<T>,
    index: usize,
}

/// Single errors types.
#[derive(Debug, PartialEq)]
pub enum SingleError<'domain, T: LabelMembership> {
    /// Invalid label index range.
    InvalidIndex {
        domain: &'domain Qualitative<T>,
        index: usize,
    },
    /// Invalid label name.
    InvalidName {
        domain: &'domain Qualitative<T>,
        name: String,
    },
}

// Note: + Display added because clion doesn't detect here correctly the trait_alias feature
impl<'domain, T: LabelMembership> Display for SingleError<'domain, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use SingleError::*;
        match &self {
            InvalidIndex { domain, index } => {
                write!(
                    f,
                    "Invalid label index {} (domain cardinality == {}).",
                    index,
                    domain.cardinality()
                )
            }
            InvalidName { domain, name } => {
                write!(
                    f,
                    "Invalid label name '{}' (domain labels are == {:?}).",
                    name,
                    domain.get_labels_names()
                )
            }
        }
    }
}

impl<'domain, T: LabelMembership> Linguistic for Single<'domain, T> {}
impl<'domain, T: LabelMembership> Valuation for Single<'domain, T> {}

impl<'domain, T: LabelMembership> Single<'domain, T> {
    /// Creates a new valuation given label `index` in `domain`.
    ///
    /// # Arguments
    /// * `domain`: A qualitative domain reference.
    /// * `index`: Label index in `domain`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Single;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert!(Single::new_by_label_index(&domain, 0).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// **SingleError::InvalidIndex**: If `index >= domain.cardinality()`.
    ///
    /// ```
    /// # use assessment::valuation::{Single, SingleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     Single::new_by_label_index(&domain, 2),
    ///     Err(SingleError::InvalidIndex { domain: &domain, index: 2 })
    /// );
    /// ```
    pub fn new_by_label_index(
        domain: &'domain Qualitative<T>,
        index: usize,
    ) -> Result<Self, SingleError<'domain, T>> {
        use SingleError::*;
        if index >= domain.cardinality() {
            Err(InvalidIndex { domain, index })
        } else {
            Ok(Self { domain, index })
        }
    }

    /// Creates a new valuation given label `name` of a label in `domain`.
    ///
    /// # Arguments
    /// * `domain`: A qualitative domain reference.
    /// * `name`: Label `name`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Single;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert!(Single::new_by_label_name(&domain, "a").is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// **SingleError::InvalidName**: If `name` isn't contained in domain's labels.
    ///
    /// ```
    /// # use assessment::valuation::{Single, SingleError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for v in ["c", "A", " a"] {
    ///     assert_eq!(
    ///         Single::new_by_label_name(&domain, v),
    ///         Err(SingleError::InvalidName { domain: &domain, name: String::from(v) })
    ///     );
    /// }
    /// ```
    pub fn new_by_label_name(
        domain: &'domain Qualitative<T>,
        name: &str,
    ) -> Result<Self, SingleError<'domain, T>> {
        use SingleError::*;
        if let Some(index) = domain.label_index(name) {
            Ok(Self { domain, index })
        } else {
            Err(InvalidName {
                domain,
                name: String::from(name),
            })
        }
    }

    /// Returns associated valuation index in domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Single;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (e, v) in [
    ///     (Single::new_by_label_index(&domain, 0), 0),
    ///     (Single::new_by_label_index(&domain, 1), 1),
    ///     (Single::new_by_label_name(&domain, "a"), 0),
    ///     (Single::new_by_label_name(&domain, "b"), 1)
    /// ] {
    ///     assert_eq!(e.unwrap().index(), v);
    /// }
    /// ```
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns associated valuation label in domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Single;
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (e, v) in [
    ///     (Single::new_by_label_index(&domain, 0), 0),
    ///     (Single::new_by_label_index(&domain, 1), 1),
    ///     (Single::new_by_label_name(&domain, "a"), 0),
    ///     (Single::new_by_label_name(&domain, "b"), 1)
    /// ] {
    ///     assert_eq!(e.unwrap().label(), domain.get_label_by_index(v).unwrap());
    /// }
    /// ```
    pub fn label(&self) -> &Label<T> {
        &self.domain.get_label_by_index(self.index).unwrap()
    }

    /// Returns valuation domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::Single;
    /// # use assessment::qualitative_domain;
    /// # use assessment::Valuation;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(*Single::new_by_label_index(&domain, 0).unwrap().domain(), domain);
    /// ```
    pub fn domain(&self) -> &'domain Qualitative<T> {
        self.domain
    }
}

impl<'domain> Single<'domain, Trapezoidal> {
    /// Unification of a Single valuation in a new domain.
    ///
    /// # Arguments
    /// * `domain`: Domain in which perform the unification.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// # use assessment::valuation::{Single, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 0.5],
    ///     "b" => vec![0.0, 0.5, 1.0],
    ///     "c" => vec![0.5, 1.0, 1.0]
    /// ].unwrap();
    /// let unification_domain = qualitative_domain![
    ///     "a" => vec![0.00, 0.00, 0.25],
    ///     "b" => vec![0.00, 0.25, 0.50],
    ///     "c" => vec![0.25, 0.50, 0.75],
    ///     "d" => vec![0.50, 0.75, 1.00],
    ///     "e" => vec![0.75, 1.00, 1.00]
    /// ].unwrap();
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
    /// ```
    /// # use assessment::qualitative_domain;
    /// # use assessment::valuation::{Single, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 0.5],
    ///     "b" => vec![0.0, 0.5, 1.0],
    ///     "c" => vec![0.5, 1.0, 1.0]
    /// ].unwrap();
    /// let unification_domain = qualitative_domain![
    ///     "a" => vec![0.00, 0.00, 0.25],
    ///     "b" => vec![0.00, 0.25, 0.50],
    ///     "c" => vec![0.25, 0.50, 0.75],
    ///     "d" => vec![0.50, 0.75, 1.00]
    /// ].unwrap();
    ///
    /// let valuation = Single::new_by_label_index(&domain, 1).unwrap();
    /// assert_eq!(
    ///     valuation.unification_in_domain(&unification_domain),
    ///     Err(UnifiedError::NonBLTSDomain { domain: &unification_domain })
    /// );
    /// ```
    pub fn unification_in_domain(
        &self,
        domain: &'domain Qualitative<Trapezoidal>,
    ) -> Result<Unified, UnifiedError<'domain>> {
        let mut measures: Vec<f32> = vec![0.; domain.cardinality()];
        measures[self.index * (domain.cardinality() - 1) / (self.domain.cardinality() - 1)] = 1.;
        Unified::new(&domain, measures)
    }

    /// Transform into a Single valuation in a different domain.
    ///
    /// # Arguments
    /// * `domain`: Domain to be used.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// # use assessment::valuation::{Single, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 0.5],
    ///     "b" => vec![0.0, 0.5, 1.0],
    ///     "c" => vec![0.5, 1.0, 1.0]
    /// ].unwrap();
    /// let unification_domain = qualitative_domain![
    ///     "a" => vec![0.00, 0.00, 0.25],
    ///     "b" => vec![0.00, 0.25, 0.50],
    ///     "c" => vec![0.25, 0.50, 0.75],
    ///     "d" => vec![0.50, 0.75, 1.00],
    ///     "e" => vec![0.75, 1.00, 1.00]
    /// ].unwrap();
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
    /// ```
    /// # use assessment::qualitative_domain;
    /// # use assessment::valuation::{Single, Unified, UnifiedError};
    /// # use assessment::utilities;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 0.5],
    ///     "b" => vec![0.0, 0.5, 1.0],
    ///     "c" => vec![0.5, 1.0, 1.0]
    /// ].unwrap();
    /// let unification_domain = qualitative_domain![
    ///     "a" => vec![0.00, 0.00, 0.25],
    ///     "b" => vec![0.00, 0.25, 0.50],
    ///     "c" => vec![0.25, 0.50, 0.75],
    ///     "d" => vec![0.50, 0.75, 1.00]
    /// ].unwrap();
    ///
    /// let valuation = Single::new_by_label_index(&domain, 1).unwrap();
    /// assert_eq!(
    ///     valuation.transform_in_domain(&unification_domain),
    ///     Err(UnifiedError::NonBLTSDomain { domain: &unification_domain })
    /// );
    /// ```
    pub fn transform_in_domain(
        &self,
        domain: &'domain Qualitative<Trapezoidal>,
    ) -> Result<Self, UnifiedError<'domain>> {
        if !domain.is_blts() {
            Err(UnifiedError::NonBLTSDomain { domain: &domain })
        } else {
            Ok(Single::new_by_label_index(
                &domain,
                self.index * (domain.cardinality() - 1) / (self.domain.cardinality() - 1),
            )
            .unwrap())
        }
    }
}

/// Generates a Unified valuation from a Linguistic valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::valuation::{Single, Unified, UnifiedError};
/// # use assessment::utilities;
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0],
///     "c" => vec![0.5, 1.0, 1.0]
/// ].unwrap();
///
/// let valuation = Single::new_by_label_index(&domain, 1).unwrap();
/// let unified = Unified::try_from(valuation).unwrap();
/// assert_eq!(*unified.measures(), vec![0.0, 1.0, 0.0]);
/// ```
///
/// # Errors
///
/// **UnifiedError::NonBLTSDomain**: If valuation domain is a Non-BLTS domain.
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::valuation::{Single, Unified, UnifiedError};
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0]
/// ].unwrap();
///
/// let valuation = Single::new_by_label_index(&domain, 1).unwrap();
/// assert_eq!(
///     Unified::try_from(valuation),
///     Err(UnifiedError::NonBLTSDomain { domain: &domain })
/// );
/// ```
///
impl<'domain> TryFrom<Single<'domain, Trapezoidal>> for Unified<'domain> {
    type Error = UnifiedError<'domain>;

    fn try_from(value: Single<'domain, Trapezoidal>) -> Result<Self, Self::Error> {
        let mut measures: Vec<f32> = vec![0.; value.domain.cardinality()];
        measures[value.index()] = 1.;
        Unified::new(&value.domain(), measures)
    }
}

/// Generates a Unified valuation from a &Linguistic valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::valuation::{Single, Unified, UnifiedError};
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0],
///     "c" => vec![0.5, 1.0, 1.0]
/// ].unwrap();
///
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
/// # use assessment::qualitative_domain;
/// # use assessment::valuation::{Single, Unified, UnifiedError};
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0]
/// ].unwrap();
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
        let mut measures: Vec<f32> = vec![0.; value.domain.cardinality()];
        measures[value.index()] = 1.;
        Unified::new(&value.domain(), measures)
    }
}

/// Generates a Single valuation from an Unified valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::utilities;
/// # use assessment::valuation::{Single, Unified, UnifiedError};
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0],
///     "c" => vec![0.5, 1.0, 1.0]
/// ].unwrap();
///
/// let valuation = Unified::new(&domain, vec![0.0, 0.7, 0.3]).unwrap();
/// let chi = valuation.chi();
/// let single = Single::try_from(valuation).unwrap();
/// assert_eq!(single.index(), 1);
/// ```
impl<'domain> TryFrom<Unified<'domain>> for Single<'domain, Trapezoidal> {
    type Error = SingleError<'domain, Trapezoidal>;

    fn try_from(value: Unified<'domain>) -> Result<Self, Self::Error> {
        Single::new_by_label_index(value.domain(), value.chi().round() as usize)
    }
}

/// Generates a Single valuation from an &Unified valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::utilities;
/// # use assessment::valuation::{Single, Unified, UnifiedError};
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0],
///     "c" => vec![0.5, 1.0, 1.0]
/// ].unwrap();
///
/// let valuation = Unified::new(&domain, vec![0.0, 0.7, 0.3]).unwrap();
/// let single = Single::try_from(&valuation).unwrap();
/// assert_eq!(single.index(), 1);
/// ```
impl<'domain> TryFrom<&Unified<'domain>> for Single<'domain, Trapezoidal> {
    type Error = SingleError<'domain, Trapezoidal>;

    fn try_from(value: &Unified<'domain>) -> Result<Self, Self::Error> {
        Single::new_by_label_index(value.domain(), value.chi().round() as usize)
    }
}

/// Generates a Single valuation from a TwoTuple valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::utilities;
/// # use assessment::valuation::{Single, TwoTuple, SingleError};
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0],
///     "c" => vec![0.5, 1.0, 1.0]
/// ].unwrap();
///
/// let valuation = TwoTuple::new_by_label_index(&domain, 1, 0.3).unwrap();
/// let single = Single::try_from(valuation).unwrap();
/// assert_eq!(single.index(), 1);
/// ```
impl<'domain, T: LabelMembership + Display> TryFrom<TwoTuple<'domain, T>> for Single<'domain, T> {
    type Error = SingleError<'domain, T>;

    fn try_from(value: TwoTuple<'domain, T>) -> Result<Self, Self::Error> {
        Single::new_by_label_index(value.domain(), value.index())
    }
}

/// Generates a Single valuation from an &TwoTuple valuation.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_domain;
/// # use assessment::utilities;
/// # use assessment::valuation::{Single, TwoTuple, SingleError};
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0],
///     "c" => vec![0.5, 1.0, 1.0]
/// ].unwrap();
///
/// let valuation = TwoTuple::new_by_label_index(&domain, 1, 0.3).unwrap();
/// let single = Single::try_from(&valuation).unwrap();
/// assert_eq!(single.index(), 1);
/// ```
impl<'domain, T: LabelMembership + Display> TryFrom<&TwoTuple<'domain, T>> for Single<'domain, T> {
    type Error = SingleError<'domain, T>;

    fn try_from(value: &TwoTuple<'domain, T>) -> Result<Self, Self::Error> {
        Single::new_by_label_index(value.domain(), value.index())
    }
}
