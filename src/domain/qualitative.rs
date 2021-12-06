use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use crate::fuzzy::label::{Label, LabelMembership};

use super::Domain;

/// Qualitative domains.
#[derive(Debug, PartialEq)]
pub struct Qualitative<T: LabelMembership> {
    labels: Vec<Label<T>>,
}

/// Qualitative errors types.
#[derive(Debug, PartialEq)]
pub enum QualitativeError {
    /// Duplicate label name.
    DuplicateName { name: String },
}

// Note: + Display added because clion doesn't detect here correctly the trait_alias feature
impl Display for QualitativeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            QualitativeError::DuplicateName { name } => {
                write!(f, "Duplicate label name {}.", name)
            }
        }
    }
}
impl<T: LabelMembership> Domain for Qualitative<T> {}

// Note: + Display added because clion doesn't detect here correctly the trait_alias feature
impl<T: LabelMembership + Display> Display for Qualitative<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.labels
                .iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl<T: LabelMembership> Qualitative<T> {
    /// Force that there are no duplicate labels names.
    fn _find_duplicate(labels: &Vec<&str>) -> Option<String> {
        let mut set = HashSet::new();
        for label in labels {
            if set.contains(label) {
                return Some(String::from(*label));
            }
            set.insert(label);
        }
        None
    }

    /// Returns labels names.
    fn _get_labels_names(labels: &Vec<Label<T>>) -> Vec<&str> {
        labels
            .iter()
            .map(|l| l.name().as_str())
            .collect::<Vec<&str>>()
    }

    /// Qualitative domain constructor.
    ///
    /// # Arguments
    /// * `labels`: Domain labels.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::domain::Qualitative;
    /// # use assessment::fuzzy::label::Label;
    /// # use assessment::fuzzy::membership::Trapezoidal;
    /// # use assessment::trapezoidal_labels;
    /// // Empty
    /// assert!(Qualitative::new(Vec::<Label<Trapezoidal>>::new()).is_ok());
    ///
    /// // Or with a vector of (valid) labels
    /// let labels = trapezoidal_labels![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    /// assert!(Qualitative::new(labels).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// **QualitativeError::DuplicateName**: If there are labels with duplicate names.
    ///
    /// ```
    /// # use assessment::domain::Qualitative;
    /// # use assessment::domain::qualitative::QualitativeError;
    /// # use assessment::trapezoidal_labels;
    /// let labels = trapezoidal_labels![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "a" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert_eq!(
    ///     Qualitative::new(labels),
    ///     Err(QualitativeError::DuplicateName { name: "a".to_string() })
    /// );
    /// ```
    ///
    pub fn new(labels: Vec<Label<T>>) -> Result<Self, QualitativeError> {
        if let Some(name) =
            Qualitative::<T>::_find_duplicate(&Qualitative::<T>::_get_labels_names(&labels))
        {
            Err(QualitativeError::DuplicateName { name })
        } else {
            Ok(Qualitative { labels })
        }
    }

    /// Returns the number of labels.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![].unwrap();
    /// assert_eq!(domain.cardinality(), 0);
    /// ```
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    /// assert_eq!(domain.cardinality(), 2);
    /// ```
    ///
    pub fn cardinality(&self) -> usize {
        self.labels.len()
    }

    /// Check if domains contains a given label name.
    ///
    /// # Arguments
    /// * `name`: Label name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (v, e) in [
    ///     ("a", true),
    ///     ("b", true),
    ///     ("c", false)
    /// ] {
    ///     assert_eq!(domain.contains_label(v), e);
    /// }
    /// ```
    pub fn contains_label(&self, name: &str) -> bool {
        Qualitative::_get_labels_names(&self.labels).contains(&name)
    }

    /// Returns label position if there is a label which this name.
    ///
    /// # Arguments
    /// * `name`: Label name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// let domain = qualitative_domain![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// for (v, e) in [
    ///     ("a", Some(0)),
    ///     ("b", Some(1)),
    ///     ("c", None)
    /// ] {
    ///     assert_eq!(domain.label_position(v), e);
    /// }
    /// ```
    pub fn label_position(&self, name: &str) -> Option<usize> {
        Qualitative::_get_labels_names(&self.labels)
            .iter()
            .position(|&v| v.eq(name))
    }

    /// Get a label given its position.
    ///
    /// # Arguments
    /// * `position`: Label position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::trapezoidal_labels;
    /// # use assessment::domain::Qualitative;
    /// let labels = trapezoidal_labels![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// let domain = Qualitative::new(labels.to_vec()).unwrap();
    ///
    /// for (v, e) in [
    ///     (0, Some(&labels[0])),
    ///     (1, Some(&labels[1])),
    ///     (2, None)
    /// ] {
    ///     assert_eq!(domain.get_label_by_position(v), e);
    /// }
    /// ```
    pub fn get_label_by_position(&self, position: usize) -> Option<&Label<T>> {
        if position < self.labels.len() {
            Some(&self.labels[position])
        } else {
            None
        }
    }

    /// Get a label given its name.
    ///
    /// # Arguments
    /// * `name`: Label name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::trapezoidal_labels;
    /// # use assessment::domain::Qualitative;
    /// let labels = trapezoidal_labels![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// let domain = Qualitative::new(labels.to_vec()).unwrap();
    ///
    /// for (v, e) in [
    ///     ("a", Some(&labels[0])),
    ///     ("b", Some(&labels[1])),
    ///     ("c", None)
    /// ] {
    ///     assert_eq!(domain.get_label_by_name(v), e);
    /// }
    /// ```
    ///
    pub fn get_label_by_name(&self, name: &str) -> Option<&Label<T>> {
        if let Some(position) = self.label_position(name) {
            Some(&self.labels[position])
        } else {
            None
        }
    }
}

#[allow(unused_imports)]
use crate::fuzzy::membership::Trapezoidal;
/// Qualitative domain.
///
/// Generates a qualitative domain. Note it is a wrapper of trapezoidal_labels macro.
///
/// # Examples
///
/// ```
/// # use assessment::qualitative_domain;
/// let domain = qualitative_domain![
///     "a" => vec![0.0, 0.0, 1.0],
///     "b" => vec![0.0, 1.0, 1.0]
/// ].unwrap();
///
/// assert_eq!(
///     format!("{}", domain),
///     "[a => (0.00, 0.00, 1.00), b => (0.00, 1.00, 1.00)]"
/// );
/// ```
///
/// # Errors
///
/// **&str**: If any label name is invalid ([Label::new]).
///
/// ```
/// # use assessment::qualitative_domain;
/// assert!(
///     qualitative_domain![
///         " a" => vec![0.0, 0.0, 1.0]
///     ].is_err()
/// );
/// ```
///
/// **&str**: If any label limits are invalid (see [Trapezoidal::new]).
///
/// ```
/// # use assessment::qualitative_domain;
/// assert!(
///     qualitative_domain![
///         "a" => vec![0.0, 0.0, 1.0, 1.0, 1.0]
///     ].is_err()
/// );
/// ```
///
/// **&str**: If labels are invalid (see [Qualitative::new]).
///
/// ```
/// # use assessment::qualitative_domain;
/// assert!(
///     qualitative_domain![
///         "a" => vec![0.0, 0.0, 1.0],
///         "a" => vec![0.0, 1.0, 1.0]
///     ].is_err()
/// );
/// ```
#[macro_export]
macro_rules! qualitative_domain {
    ( $( $name:expr => $membership:expr ),* ) => {
        {
            use assessment::trapezoidal_labels;
            use assessment::domain::Qualitative;
            match trapezoidal_labels![$( $name => $membership ),*] {
                Ok(labels) => {
                    match Qualitative::new(labels) {
                        Ok(domain) => Ok(domain),
                        Err(e) => Err(format!("{}", e)),
                    }
                },
                Err(e) => Err(format!("{}", e))
            }
        }
    }
}
