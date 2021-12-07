use crate::domain::Qualitative;
use crate::fuzzy::{Label, LabelMembership};
use crate::valuation::Linguistic;
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
