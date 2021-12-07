use crate::domain::Qualitative;
use crate::fuzzy::LabelMembership;
use crate::valuation::Linguistic;
use crate::Valuation;
use std::fmt::{Display, Formatter};

/// Hesitant linguistic valuation.
#[derive(Debug, PartialEq)]
pub struct Hesitant<'domain, T: LabelMembership> {
    relation: HesitantRelation<'domain, T>,
}

/// Hesitant relation type.
#[derive(Debug, PartialEq)]
pub enum HesitantRelation<'domain, T: LabelMembership> {
    SingleValue {
        domain: &'domain Qualitative<T>,
        index: usize,
    },
    AtLeast {
        domain: &'domain Qualitative<T>,
        index: usize,
    },
    AtMost {
        domain: &'domain Qualitative<T>,
        index: usize,
    },
    Between {
        domain: &'domain Qualitative<T>,
        lower: usize,
        upper: usize,
    },
    LowerThan {
        domain: &'domain Qualitative<T>,
        index: usize,
    },
    GreaterThan {
        domain: &'domain Qualitative<T>,
        index: usize,
    },
}

impl<'domain, T: LabelMembership> HesitantRelation<'domain, T> {
    /// Validate hesitant relation.
    ///
    /// Note: function takes ownership of **self**.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::{HesitantRelation, HesitantError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (relation) in [
    ///     HesitantRelation::SingleValue { domain: &domain, index: 0 },
    ///     HesitantRelation::SingleValue { domain: &domain, index: 1 },
    ///     HesitantRelation::AtLeast { domain: &domain, index: 0 },
    ///     HesitantRelation::AtLeast { domain: &domain, index: 1 },
    ///     HesitantRelation::AtMost { domain: &domain, index: 0 },
    ///     HesitantRelation::AtMost { domain: &domain, index: 1 },
    ///     HesitantRelation::LowerThan { domain: &domain, index: 1 },
    ///     HesitantRelation::GreaterThan { domain: &domain, index: 0 },
    ///     HesitantRelation::Between { domain: &domain, lower: 0, upper: 1 },
    /// ] {
    ///     assert!(relation.validate().is_ok());
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// ## SingleValue
    ///
    /// **HesitantError::InvalidIndex**: If `index >= domain.cardinality()`.
    ///
    /// ```
    /// # use assessment::valuation::{HesitantRelation, HesitantError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     HesitantRelation::SingleValue { domain: &domain, index: 2 }.validate(),
    ///     Err(HesitantError::InvalidIndex { domain: &domain, index: 2 }),
    /// );
    /// ```
    ///
    /// ## AtLeast
    ///
    /// **HesitantError::InvalidIndex**: If `index >= domain.cardinality()`.
    ///
    /// ```
    /// # use assessment::valuation::{HesitantRelation, HesitantError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     HesitantRelation::AtLeast { domain: &domain, index: 2 }.validate(),
    ///     Err(HesitantError::InvalidIndex { domain: &domain, index: 2 }),
    /// );
    /// ```
    ///
    /// ## AtMost
    ///
    /// **HesitantError::InvalidIndex**: If `index >= domain.cardinality()`.
    ///
    /// ```
    /// # use assessment::valuation::{HesitantRelation, HesitantError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     HesitantRelation::AtMost { domain: &domain, index: 2 }.validate(),
    ///     Err(HesitantError::InvalidIndex { domain: &domain, index: 2 }),
    /// );
    /// ```
    ///
    /// ## LowerThan
    ///
    /// **HesitantError::InvalidIndex**: If `index >= domain.cardinality()`.
    ///
    /// ```
    /// # use assessment::valuation::{HesitantRelation, HesitantError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     HesitantRelation::LowerThan { domain: &domain, index: 2 }.validate(),
    ///     Err(HesitantError::InvalidIndex { domain: &domain, index: 2 }),
    /// );
    /// ```
    ///
    /// **HesitantError::InvalidIndex**: If `index == 0`.
    ///
    /// ```
    /// # use assessment::valuation::{HesitantRelation, HesitantError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     HesitantRelation::LowerThan { domain: &domain, index: 0 }.validate(),
    ///     Err(HesitantError::InvalidIndex { domain: &domain, index: 0 }),
    /// );
    /// ```
    ///
    /// ## GreaterThan
    ///
    /// **HesitantError::InvalidIndex**: If `index >= domain.cardinality() - 1`.
    ///
    /// ```
    /// # use assessment::valuation::{HesitantRelation, HesitantError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     HesitantRelation::GreaterThan { domain: &domain, index: 1 }.validate(),
    ///     Err(HesitantError::InvalidIndex { domain: &domain, index: 1 }),
    /// );
    /// ```
    ///
    /// ## Between
    ///
    /// **HesitantError::InvalidIndex**: If `upper >= domain.cardinality()`.
    ///
    /// ```
    /// # use assessment::valuation::{HesitantRelation, HesitantError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     HesitantRelation::Between { domain: &domain, lower: 0, upper: 2 }.validate(),
    ///     Err(HesitantError::InvalidIndex { domain: &domain, index: 2 }),
    /// );
    /// ```
    ///
    /// **HesitantError::InvalidRange**: If `lower >= upper`.
    ///
    /// ```
    /// # use assessment::valuation::{HesitantRelation, HesitantError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     HesitantRelation::Between { domain: &domain, lower: 1, upper: 1 }.validate(),
    ///     Err(HesitantError::InvalidRange { lower: 1, upper: 1 }),
    /// );
    /// ```
    pub fn validate(self) -> Result<Self, HesitantError<'domain, T>> {
        use HesitantError::*;
        use HesitantRelation::*;
        match self {
            SingleValue { domain, index }
            | AtLeast { domain, index }
            | AtMost { domain, index } => {
                if index >= domain.cardinality() {
                    Err(InvalidIndex { domain, index })
                } else {
                    Ok(self)
                }
            }
            LowerThan { domain, index } => {
                if index == 0 || index >= domain.cardinality() {
                    Err(InvalidIndex { domain, index })
                } else {
                    Ok(self)
                }
            }
            GreaterThan { domain, index } => {
                if index >= domain.cardinality() - 1 {
                    Err(InvalidIndex { domain, index })
                } else {
                    Ok(self)
                }
            }
            Between {
                domain,
                lower,
                upper,
            } => {
                if lower >= upper {
                    Err(InvalidRange { lower, upper })
                } else if upper >= domain.cardinality() {
                    Err(InvalidIndex {
                        domain,
                        index: upper,
                    })
                } else {
                    Ok(self)
                }
            }
        }
    }

    /// Returns relation domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::{HesitantRelation, HesitantError};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (relation) in [
    ///     HesitantRelation::SingleValue { domain: &domain, index: 0 },
    ///     HesitantRelation::SingleValue { domain: &domain, index: 1 },
    ///     HesitantRelation::AtLeast { domain: &domain, index: 0 },
    ///     HesitantRelation::AtLeast { domain: &domain, index: 1 },
    ///     HesitantRelation::AtMost { domain: &domain, index: 0 },
    ///     HesitantRelation::AtMost { domain: &domain, index: 1 },
    ///     HesitantRelation::LowerThan { domain: &domain, index: 1 },
    ///     HesitantRelation::GreaterThan { domain: &domain, index: 0 },
    ///     HesitantRelation::Between { domain: &domain, lower: 0, upper: 1 },
    /// ] {
    ///     assert_eq!(*relation.domain(), domain);
    /// }
    /// ```
    pub fn domain(&self) -> &'domain Qualitative<T> {
        use HesitantRelation::*;
        match self {
            SingleValue { domain, index: _ } => domain,
            AtLeast { domain, index: _ } => domain,
            AtMost { domain, index: _ } => domain,
            LowerThan { domain, index: _ } => domain,
            GreaterThan { domain, index: _ } => domain,
            Between {
                domain,
                lower: _,
                upper: _,
            } => domain,
        }
    }
}

/// Hesitant errors types.
#[derive(Debug, PartialEq)]
pub enum HesitantError<'domain, T: LabelMembership> {
    /// Invalid label index range.
    InvalidIndex {
        domain: &'domain Qualitative<T>,
        index: usize,
    },
    /// Invalid label range.
    InvalidRange { lower: usize, upper: usize },
    /// Invalid label name.
    InvalidName {
        domain: &'domain Qualitative<T>,
        name: String,
    },
}

impl<'domain, T: LabelMembership> Linguistic for Hesitant<'domain, T> {}
impl<'domain, T: LabelMembership> Valuation for Hesitant<'domain, T> {}

// Note: + Display added because clion doesn't detect here correctly the trait_alias feature
impl<'domain, T: LabelMembership> Display for HesitantError<'domain, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use HesitantError::*;
        match &self {
            InvalidIndex { domain, index } => {
                write!(
                    f,
                    "Invalid label index {} (domain cardinality == {}).",
                    index,
                    domain.cardinality()
                )
            }
            InvalidRange { lower, upper } => {
                write!(f, "Invalid label range [{}-{}].", lower, upper)
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

impl<'domain, T: LabelMembership> Hesitant<'domain, T> {
    /// Creates a new valuation given label `index` in `domain`.
    ///
    /// # Arguments
    /// * `relation`: A hesitant relation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::{Hesitant, HesitantError, HesitantRelation};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (relation) in [
    ///     HesitantRelation::SingleValue { domain: &domain, index: 0 },
    ///     HesitantRelation::SingleValue { domain: &domain, index: 1 },
    ///     HesitantRelation::AtLeast { domain: &domain, index: 0 },
    ///     HesitantRelation::AtLeast { domain: &domain, index: 1 },
    ///     HesitantRelation::AtMost { domain: &domain, index: 0 },
    ///     HesitantRelation::AtMost { domain: &domain, index: 1 },
    ///     HesitantRelation::LowerThan { domain: &domain, index: 1 },
    ///     HesitantRelation::GreaterThan { domain: &domain, index: 0 },
    ///     HesitantRelation::Between { domain: &domain, lower: 0, upper: 1 },
    /// ] {
    ///     assert!(Hesitant::new(relation).is_ok());
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// See [HesitantRelation::validate].
    ///
    pub fn new(relation: HesitantRelation<'domain, T>) -> Result<Self, HesitantError<'domain, T>> {
        let relation = relation.validate()?;
        Ok(Self { relation })
    }

    /// Returns associated valuation indexes in domain.
    ///
    /// It returns (first index, last index): (usize, usize).
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::{Hesitant, HesitantError, HesitantRelation};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (indexes, relation) in [
    ///     ((0, 0), HesitantRelation::SingleValue { domain: &domain, index: 0 }),
    ///     ((1, 1), HesitantRelation::SingleValue { domain: &domain, index: 1 }),
    ///     ((0, 1), HesitantRelation::AtLeast { domain: &domain, index: 0 }),
    ///     ((1, 1), HesitantRelation::AtLeast { domain: &domain, index: 1 }),
    ///     ((0, 0), HesitantRelation::AtMost { domain: &domain, index: 0 }),
    ///     ((0, 1), HesitantRelation::AtMost { domain: &domain, index: 1 }),
    ///     ((0, 0), HesitantRelation::LowerThan { domain: &domain, index: 1 }),
    ///     ((1, 1), HesitantRelation::GreaterThan { domain: &domain, index: 0 }),
    ///     ((0, 1), HesitantRelation::Between { domain: &domain, lower: 0, upper: 1 }),
    /// ] {
    ///     assert_eq!(indexes, Hesitant::new(relation).unwrap().indexes());
    /// }
    /// ```
    pub fn indexes(&self) -> (usize, usize) {
        use HesitantRelation::*;
        match self.relation {
            SingleValue { domain: _, index } => (index, index),
            AtLeast { domain, index } => (index, domain.cardinality() - 1),
            AtMost { domain: _, index } => (0, index),
            LowerThan { domain: _, index } => (0, index - 1),
            GreaterThan { domain, index } => (index + 1, domain.cardinality() - 1),
            Between {
                domain: _,
                lower,
                upper,
            } => (lower, upper),
        }
    }

    /// Returns associated valuation labels names in domain.
    ///
    /// It returns (first label, last label): (String, String).
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::{Hesitant, HesitantError, HesitantRelation};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (labels, relation) in [
    ///     (("a", "a"), HesitantRelation::SingleValue { domain: &domain, index: 0 }),
    ///     (("b", "b"), HesitantRelation::SingleValue { domain: &domain, index: 1 }),
    ///     (("a", "b"), HesitantRelation::AtLeast { domain: &domain, index: 0 }),
    ///     (("b", "b"), HesitantRelation::AtLeast { domain: &domain, index: 1 }),
    ///     (("a", "a"), HesitantRelation::AtMost { domain: &domain, index: 0 }),
    ///     (("a", "b"), HesitantRelation::AtMost { domain: &domain, index: 1 }),
    ///     (("a", "a"), HesitantRelation::LowerThan { domain: &domain, index: 1 }),
    ///     (("b", "b"), HesitantRelation::GreaterThan { domain: &domain, index: 0 }),
    ///     (("a", "b"), HesitantRelation::Between { domain: &domain, lower: 0, upper: 1 }),
    /// ] {
    ///     assert_eq!(
    ///         (labels.0.to_string(), labels.1.to_string()),
    ///         Hesitant::new(relation).unwrap().labels()
    ///     );
    /// }
    /// ```
    pub fn labels(&self) -> (String, String) {
        let (lower, upper) = self.indexes();
        let domain = self.relation.domain();
        (
            domain.get_label_by_index(lower).unwrap().name().clone(),
            domain.get_label_by_index(upper).unwrap().name().clone(),
        )
    }

    /// Returns valuation domain.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::valuation::{Hesitant, HesitantRelation};
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     *Hesitant::new(HesitantRelation::SingleValue { domain: &domain, index: 0 })
    ///         .unwrap()
    ///         .domain(),
    ///     domain
    /// );
    /// ```
    pub fn domain(&self) -> &'domain Qualitative<T> {
        self.relation.domain()
    }
}
