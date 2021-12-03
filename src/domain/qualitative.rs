use super::Domain;
use crate::fuzzy::label::Label;
use std::collections::HashSet;

/// Qualitative domains
pub struct Qualitative {
    labels: Vec<Label>,
}

// // //
// Traits implementations
//

impl Domain for Qualitative {}

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
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let mut labels = Vec::new();
    ///
    /// labels.push(
    ///     Label::new(
    ///         String::from("a"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.5, 1.0]))
    ///     )
    /// );
    ///
    /// assessment::domain::Qualitative::new(labels);
    /// ```
    ///
    /// ```
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let mut labels = Vec::new();
    ///
    /// labels.push(
    ///     Label::new(
    ///         String::from("a"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.0, 1.0]))
    ///     )
    /// );
    /// labels.push(
    ///     Label::new(
    ///         String::from("b"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 1.0, 1.0]))
    ///     )
    /// );
    ///
    /// assessment::domain::Qualitative::new(labels);
    /// ```
    ///
    /// # Panics
    ///
    /// If there are labels with duplicate names
    ///
    /// ```should_panic
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let mut labels = Vec::new();
    ///
    /// labels.push(
    ///     Label::new(
    ///         String::from("a"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.0, 1.0]))
    ///     )
    /// );
    /// labels.push(
    ///     Label::new(
    ///         String::from("a"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 1.0, 1.0]))
    ///     )
    /// );
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
    /// let labels = Vec::new();
    /// let domain = assessment::domain::Qualitative::new(labels);
    ///
    /// assert_eq!(domain.cardinality(), 0);
    /// ```
    ///
    /// ```
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let mut labels = Vec::new();
    ///
    /// labels.push(
    ///     Label::new(
    ///         String::from("a"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.0, 1.0]))
    ///     )
    /// );
    /// labels.push(
    ///     Label::new(
    ///         String::from("b"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 1.0, 1.0]))
    ///     )
    /// );
    ///
    /// let domain = assessment::domain::Qualitative::new(labels);
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
    ///
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let mut labels = Vec::new();
    ///
    /// labels.push(
    ///     Label::new(
    ///         String::from("a"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.0, 1.0]))
    ///     )
    /// );
    /// labels.push(
    ///     Label::new(
    ///         String::from("b"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 1.0, 1.0]))
    ///     )
    /// );
    ///
    /// let domain = assessment::domain::Qualitative::new(labels);
    ///
    /// assert_eq!(domain.contains_label("a"), true);
    /// assert_eq!(domain.contains_label("b"), true);
    /// assert_eq!(domain.contains_label("c"), false);
    /// assert_eq!(domain.contains_label(" a"), false);
    /// assert_eq!(domain.contains_label("A"), false);
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
    ///
    /// Check if domains contains a given label name
    ///
    /// # Params
    /// - `name`: Label name.
    ///
    /// # Examples
    ///
    /// ```
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let mut labels = Vec::new();
    ///
    /// labels.push(
    ///     Label::new(
    ///         String::from("a"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.0, 1.0]))
    ///     )
    /// );
    /// labels.push(
    ///     Label::new(
    ///         String::from("b"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 1.0, 1.0]))
    ///     )
    /// );
    ///
    /// let domain = assessment::domain::Qualitative::new(labels);
    ///
    /// assert_eq!(domain.label_position("a"), Some(0));
    /// assert_eq!(domain.label_position("b"), Some(1));
    /// assert_eq!(domain.label_position("c"), None);
    /// assert_eq!(domain.label_position(" a"), None);
    /// assert_eq!(domain.label_position("A"), None);
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
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let mut labels = Vec::new();
    ///
    /// labels.push(
    ///     Label::new(
    ///         String::from("a"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.0, 1.0]))
    ///     )
    /// );
    /// labels.push(
    ///     Label::new(
    ///         String::from("b"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 1.0, 1.0]))
    ///     )
    /// );
    ///
    /// let domain = assessment::domain::Qualitative::new(labels.to_vec());
    ///
    /// assert_eq!(*domain.get_label_by_position(0), labels[0]);
    /// assert_eq!(*domain.get_label_by_position(1), labels[1]);
    /// ```
    ///
    /// # Panics
    ///
    /// If `position >= self.labels.len()`
    ///
    /// ```should_panic
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let mut labels = Vec::new();
    ///
    /// labels.push(
    ///     Label::new(
    ///         String::from("a"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.0, 1.0]))
    ///     )
    /// );
    /// labels.push(
    ///     Label::new(
    ///         String::from("b"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 1.0, 1.0]))
    ///     )
    /// );
    ///
    /// let domain = assessment::domain::Qualitative::new(labels);
    ///
    /// domain.get_label_by_position(2);
    /// ```
    ///
    pub fn get_label_by_position(&self, position: usize) -> &Label {
        &self.labels[position]
    }

    /// Get a label given its name
    ///
    /// # Params
    /// - `name`: Label name
    ///
    /// # Examples
    ///
    /// ```
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let mut labels = Vec::new();
    ///
    /// labels.push(
    ///     Label::new(
    ///         String::from("a"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.0, 1.0]))
    ///     )
    /// );
    /// labels.push(
    ///     Label::new(
    ///         String::from("b"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 1.0, 1.0]))
    ///     )
    /// );
    ///
    /// let domain = assessment::domain::Qualitative::new(labels.to_vec());
    ///
    /// assert_eq!(*domain.get_label_by_name("a"), labels[0]);
    /// assert_eq!(*domain.get_label_by_name("b"), labels[1]);
    /// ```
    ///
    /// # Panics
    ///
    /// If there is no label with the given `name`
    ///
    /// ```should_panic
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let mut labels = Vec::new();
    ///
    /// labels.push(
    ///     Label::new(
    ///         String::from("a"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.0, 1.0]))
    ///     )
    /// );
    /// labels.push(
    ///     Label::new(
    ///         String::from("b"),
    ///         SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 1.0, 1.0]))
    ///     )
    /// );
    ///
    /// let domain = assessment::domain::Qualitative::new(labels);
    ///
    /// domain.get_label_by_name(" a");
    /// ```
    ///
    pub fn get_label_by_name(&self, name: &str) -> &Label {
        if let Some(position) = self.label_position(name) {
            &self.labels[position]
        } else {
            panic!("There is no label with the name {}", name);
        }
    }
}
