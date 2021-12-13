use crate::domain::Qualitative;
use crate::fuzzy::membership::Trapezoidal;
use crate::fuzzy::{Label, LabelMembership};
use crate::valuation::{Linguistic, Unified, UnifiedError};
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
