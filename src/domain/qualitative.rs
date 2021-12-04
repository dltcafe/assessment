use super::Domain;
use crate::fuzzy::label::Label;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

/// Qualitative domains
#[derive(Debug)]
pub struct Qualitative {
    labels: Vec<Label>,
}

// // //
// Traits implementations
//

impl Domain for Qualitative {}

impl Display for Qualitative {
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

// // //
// Implementation
//

impl Qualitative {
    /// Force that there are no duplicate labels names
    fn _force_unique_labels_names(labels: &Vec<&str>) {
        let mut set = HashSet::new();
        for label in labels {
            if set.contains(label) {
                panic!("Duplicate label name {}", label);
            }
            set.insert(label);
        }
    }

    /// Returns labels names
    fn _get_labels_names(labels: &Vec<Label>) -> Vec<&str> {
        labels
            .iter()
            .map(|l| l.name().as_str())
            .collect::<Vec<&str>>()
    }

    /// Qualitative domain constructor
    ///
    /// # Params
    /// - `labels`: Domain labels
    ///
    /// # Examples
    ///
    /// ```
    /// let labels = Vec::new();
    /// assessment::domain::Qualitative::new(labels);
    /// ```
    ///
    /// ```
    /// use assessment::trapezoidal_labels;
    ///
    /// let mut labels = trapezoidal_labels![
    ///     "a" => &vec![0.0, 0.0, 1.0],
    ///     "b" => &vec![0.0, 1.0, 1.0]
    /// ];
    ///
    /// assessment::domain::Qualitative::new(labels);
    /// ```
    ///
    /// # Panics
    ///
    /// If there are labels with duplicate names
    ///
    /// ```should_panic
    /// use assessment::trapezoidal_labels;
    ///
    /// let mut labels = trapezoidal_labels![
    ///     "a" => &vec![0.0, 0.0, 1.0],
    ///     "a" => &vec![0.0, 1.0, 1.0]
    /// ];
    ///
    /// assessment::domain::Qualitative::new(labels);
    /// ```
    ///
    pub fn new(labels: Vec<Label>) -> Self {
        Qualitative::_force_unique_labels_names(&Qualitative::_get_labels_names(&labels));
        Qualitative { labels }
    }

    /// Returns the number of labels
    ///
    /// # Examples
    ///
    /// ```
    /// let domain = assessment::domain::Qualitative::new(Vec::new());
    ///
    /// assert_eq!(domain.cardinality(), 0);
    /// ```
    ///
    /// ```
    /// let domain = assessment::qualitative_domain![
    ///     "a" => &vec![0.0, 0.0, 1.0],
    ///     "b" => &vec![0.0, 1.0, 1.0]
    /// ];
    ///
    /// assert_eq!(domain.cardinality(), 2);
    /// ```
    ///
    pub fn cardinality(&self) -> usize {
        self.labels.len()
    }

    /// Check if domains contains a given label name
    ///
    /// # Params
    /// - `name`: Label name.
    ///
    /// # Examples
    ///
    /// ```
    /// let domain = assessment::qualitative_domain![
    ///     "a" => &vec![0.0, 0.0, 1.0],
    ///     "b" => &vec![0.0, 1.0, 1.0]
    /// ];
    ///
    /// for (v, e) in [("a", true), ("b", true), ("c", false)] {
    ///     assert_eq!(domain.contains_label(v), e);
    /// }
    /// ```
    pub fn contains_label(&self, name: &str) -> bool {
        Qualitative::_get_labels_names(&self.labels).contains(&name)
    }

    /// Returns label position if there is a label which this name
    ///
    /// # Params
    /// - `name`: Label name.
    ///
    /// # Examples
    ///
    /// ```
    /// let domain = assessment::qualitative_domain![
    ///     "a" => &vec![0.0, 0.0, 1.0],
    ///     "b" => &vec![0.0, 1.0, 1.0]
    /// ];
    ///
    /// for (v, e) in [("a", Some(0)), ("b", Some(1)), ("c", None)] {
    ///     assert_eq!(domain.label_position(v), e);
    /// }
    /// ```
    pub fn label_position(&self, name: &str) -> Option<usize> {
        Qualitative::_get_labels_names(&self.labels)
            .iter()
            .position(|&v| v.eq(name))
    }

    /// Get a label given its position
    ///
    /// # Params
    /// - `position`: Label position
    ///
    /// # Examples
    ///
    /// ```
    /// use assessment::trapezoidal_labels;
    ///
    /// let mut labels = trapezoidal_labels![
    ///     "a" => &vec![0.0, 0.0, 1.0],
    ///     "b" => &vec![0.0, 1.0, 1.0]
    /// ];
    ///
    /// let domain = assessment::domain::Qualitative::new(labels.to_vec());
    ///
    /// for (v, e) in [(0, Some(&labels[0])), (1, Some(&labels[1])), (2, None)] {
    ///     assert_eq!(domain.get_label_by_position(v), e);
    /// }
    /// ```
    ///
    pub fn get_label_by_position(&self, position: usize) -> Option<&Label> {
        if position < self.labels.len() {
            Some(&self.labels[position])
        } else {
            None
        }
    }

    /// Get a label given its name
    ///
    /// # Params
    /// - `name`: Label name
    ///
    /// # Examples
    ///
    /// ```
    /// use assessment::trapezoidal_labels;
    ///
    /// let mut labels = trapezoidal_labels![
    ///     "a" => &vec![0.0, 0.0, 1.0],
    ///     "b" => &vec![0.0, 1.0, 1.0]
    /// ];
    ///
    /// let domain = assessment::domain::Qualitative::new(labels.to_vec());
    ///
    /// for (v, e) in [("a", Some(&labels[0])), ("b", Some(&labels[1])), ("c", None)] {
    ///     assert_eq!(domain.get_label_by_name(v), e);
    /// }
    /// ```
    ///
    pub fn get_label_by_name(&self, name: &str) -> Option<&Label> {
        if let Some(position) = self.label_position(name) {
            Some(&self.labels[position])
        } else {
            None
        }
    }
}

// // //
// Macros
//

/// Qualitative domain
///
/// Generates a qualitative domain. Note it is a wrapper of trapezoidal_labels macro
///
/// # Examples
///
/// ```
/// let domain = assessment::qualitative_domain![
///     "a" => &vec![0.0, 0.0, 1.0],
///     "b" => &vec![0.0, 1.0, 1.0]
/// ];
///
/// assert_eq!(format!("{}", domain), "[a => (0.00, 0.00, 1.00), b => (0.00, 1.00, 1.00)]");
/// ```
///
/// # Panics
///
/// If any label name is invalid (see Label::new(&self, name))
///
/// ```should_panic
/// assessment::qualitative_domain![
///     " a" => &vec![0.0, 0.0, 1.0]
/// ];
/// ```
///
/// If any label limits are invalid (see Trapezoidal::new(&self, &limits))
///
/// ```should_panic
/// assessment::qualitative_domain![
///     "a" => &vec![0.0, 0.0, 1.0, 1.0, 1.0]
/// ];
/// ```
///
/// If labels are invalid (see Qualitative::new(&self, labels))
///
/// ```should_panic
/// assessment::qualitative_domain![
///     "a" => &vec![0.0, 0.0, 1.0],
///     "a" => &vec![0.0, 1.0, 1.0]
/// ];
/// ```
#[macro_export]
macro_rules! qualitative_domain {
    ( $( $name:expr => $membership:expr ),* ) => {
        {
            assessment::domain::Qualitative::new(
                assessment::trapezoidal_labels![
                    $( $name => $membership ),*
                ]
            )
        }
    };
}
