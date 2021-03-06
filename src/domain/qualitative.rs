use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use crate::fuzzy::membership::piecewise::{LinearFunction, PiecewiseLinearFunction};
use crate::fuzzy::membership::Trapezoidal;
use crate::fuzzy::{label::get_labels_names, Label, LabelMembership};
use crate::utilities;

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
        use QualitativeError::*;
        match &self {
            DuplicateName { name } => {
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
    /// Returns the first duplicate value.
    fn _find_duplicate(labels: &[&str]) -> Option<String> {
        let mut set = HashSet::new();
        for label in labels {
            if set.contains(label) {
                return Some(String::from(*label));
            }
            set.insert(label);
        }
        None
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
    /// # use assessment::fuzzy::{Label, membership::Trapezoidal};
    /// # use assessment::trapezoidal_labels;
    /// // Empty
    /// assert!(Qualitative::new(Vec::<Label<Trapezoidal>>::new()).is_ok());
    ///
    /// // Or with a vector of (valid) labels
    /// let labels = trapezoidal_labels![
    ///     "a" => vec![0.0, 0.0, 1.0],
    ///     "b" => vec![0.0, 1.0, 1.0]
    /// ].unwrap();
    ///
    /// assert!(Qualitative::new(labels).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// **QualitativeError::DuplicateName**: If there are labels with duplicate names.
    ///
    /// ```
    /// # use assessment::domain::{Qualitative, QualitativeError};
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
        use QualitativeError::*;
        if let Some(name) = Qualitative::<T>::_find_duplicate(&get_labels_names(&labels)) {
            Err(DuplicateName { name })
        } else {
            Ok(Self { labels })
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
        get_labels_names(&self.labels).contains(&name)
    }

    /// Returns label index if there is a label which this name.
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
    ///     assert_eq!(domain.label_index(v), e);
    /// }
    /// ```
    pub fn label_index(&self, name: &str) -> Option<usize> {
        get_labels_names(&self.labels)
            .iter()
            .position(|&v| v.eq(name))
    }

    /// Get a label given its index.
    ///
    /// # Arguments
    /// * `index`: Label index.
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
    ///     assert_eq!(domain.get_label_by_index(v), e);
    /// }
    /// ```
    pub fn get_label_by_index(&self, index: usize) -> Option<&Label<T>> {
        if index < self.labels.len() {
            Some(&self.labels[index])
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
    pub fn get_label_by_name(&self, name: &str) -> Option<&Label<T>> {
        if let Some(index) = self.label_index(name) {
            Some(&self.labels[index])
        } else {
            None
        }
    }

    /// Returns labels names.
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
    ///assert_eq!(domain.get_labels_names(), vec!["a", "b"]);
    /// ```
    pub fn get_labels_names(&self) -> Vec<&str> {
        get_labels_names(&self.labels)
    }

    /// Checks if the domain is odd.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// for (d, e) in [
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 1.0], "b" => vec![0.0, 1.0, 1.0]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.5, 1.0, 1.0]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.0, 0.5, 1.0], "c" => vec![0.5, 1.0, 1.0]], true)
    /// ] {
    ///     assert_eq!(d.unwrap().is_odd(), e);
    /// }
    /// ```
    pub fn is_odd(&self) -> bool {
        self.cardinality() % 2 != 0
    }
}

impl Qualitative<Trapezoidal> {
    /// Checks if the domain is a fuzzy partition.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// for (d, e) in [
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 1.0], "b" => vec![0.0, 1.0, 1.0]], true),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.5, 1.0, 1.0]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.0, 0.5, 1.0], "c" => vec![0.5, 1.0, 1.0]], true)
    /// ] {
    ///     assert_eq!(d.unwrap().is_fuzzy_partition(), e);
    /// }
    /// ```
    pub fn is_fuzzy_partition(&self) -> bool {
        let mut fuzzy_partition = PiecewiseLinearFunction::new();
        fuzzy_partition
            .add(0.0, 1.0, LinearFunction::new(0.0, 1.0))
            .unwrap();

        PiecewiseLinearFunction::from(self) == fuzzy_partition
    }

    /// Checks if the domain is triangular.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// for (d, e) in [
    ///     (qualitative_domain!["a" => vec![0.0, 0.25, 0.75, 1.0]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.5, 0.75, 1.0, 1.0]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5]], true),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.0, 0.5, 1.0], "c" => vec![0.5, 1.0, 1.0]], true)
    /// ] {
    ///     assert_eq!(d.unwrap().is_triangular(), e);
    /// }
    /// ```
    pub fn is_triangular(&self) -> bool {
        for l in &self.labels {
            if !l.membership().is_triangular() {
                return false;
            }
        }

        true
    }

    /// Checks if the domain is Triangular, Odd and Ruspini.
    ///
    /// Note that Ruspini eq. Fuzzy partition.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// for (d, e) in [
    ///     (qualitative_domain!["a" => vec![0.0, 0.25, 0.75, 1.0]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.5, 0.75, 1.0, 1.0]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.0, 0.5, 1.0], "c" => vec![0.5, 1.0, 1.0]], true),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.33], "b" => vec![0.0, 0.33, 0.66], "c" => vec![0.33, 0.66, 1.0], "d" => vec![0.66, 1.0, 1.0]], false)
    /// ] {
    ///     assert_eq!(d.unwrap().is_tor(), e);
    /// }
    /// ```
    pub fn is_tor(&self) -> bool {
        self.is_odd() && self.is_triangular() && self.is_fuzzy_partition()
    }

    /// Checks if the domain is symmetrical.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// for (d, e) in [
    ///     (qualitative_domain!["a" => vec![0.0, 0.25, 0.75, 1.0]], true),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.5, 1.0, 1.0]], true),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.5, 1.0]], true),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.0, 0.5, 1.0], "c" => vec![0.5, 1.0, 1.0]], true),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 1./3.], "b" => vec![0.0, 1.0/3.0, 2.0/3.0], "c" => vec![1.0/3.0, 2.0/3.0, 1.0], "d" => vec![2.0/3.0, 1.0, 1.0]], true)
    /// ] {
    ///     assert_eq!(d.unwrap().is_symmetrical(), e);
    /// }
    /// ```
    pub fn is_symmetrical(&self) -> bool {
        let cardinality = self.cardinality();

        if cardinality == 0 {
            return true;
        }

        let center_pos = cardinality / 2;
        let centroid;

        if self.is_odd() {
            let center_label = &self.labels[center_pos];
            if !center_label.membership().is_symmetrical() {
                return false;
            } else {
                centroid = center_label.membership().centroid();
            }
        } else {
            centroid = (self.labels[center_pos].membership().centroid()
                + self.labels[center_pos - 1].membership().centroid())
                / 2.;
        }

        for pos in 0..center_pos {
            if !self.labels[pos].membership().is_symmetrical_respect_center(
                self.labels[cardinality - 1 - pos].membership(),
                centroid,
            ) {
                return false;
            }
        }

        true
    }

    /// Checks if the domain is uniform.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// for (d, e) in [
    ///     (qualitative_domain!["a" => vec![0.0, 0.25, 0.75, 1.0]], true),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.5, 1.0, 1.0]], true),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.0, 0.5, 1.0], "c" => vec![0.5, 1.0, 1.0]], true),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.0, 1./3., 2./3.], "c" => vec![0.5, 1.0, 1.0]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 1./3.], "b" => vec![0.0, 1./3., 2./3.], "c" => vec![1./3., 2./3., 1.0], "d" => vec![2./3., 1.0, 1.0]], true)
    /// ] {
    ///     assert_eq!(d.unwrap().is_uniform(), e);
    /// }
    /// ```
    pub fn is_uniform(&self) -> bool {
        let cardinality = self.cardinality();
        if cardinality <= 1 {
            return true;
        }

        let compute_diff = |i: usize| {
            let (a, b) = self.labels[i].membership().center();
            let (c, d) = self.labels[i - 1].membership().center();
            (a + b - c - d) / 2.
        };

        let diff = compute_diff(1);
        for pos in 2..cardinality {
            if !utilities::math::approx_equal_f32(diff, compute_diff(pos), 5) {
                return false;
            }
        }

        true
    }

    /// Checks if the domain is a Basic Linguistic Term Set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::qualitative_domain;
    /// for (d, e) in [
    ///     (qualitative_domain!["a" => vec![0.0, 0.25, 0.75, 1.0]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.5, 1.0, 1.0]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.0, 0.5, 1.0], "c" => vec![0.5, 1.0, 1.0]], true),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 0.5], "b" => vec![0.0, 1./3., 2./3.], "c" => vec![0.5, 1.0, 1.0]], false),
    ///     (qualitative_domain!["a" => vec![0.0, 0.0, 1./3.], "b" => vec![0.0, 1./3., 2./3.], "c" => vec![1./3., 2./3., 1.0], "d" => vec![2./3., 1.0, 1.0]], false),
    ///     (qualitative_domain!["a" => vec![0., 0., 1./4.], "b" => vec![0., 1./4., 2./4.], "c" => vec![1./4., 2./4., 3./4.], "d" => vec![2./4., 3./4., 1.], "e" => vec![3./4., 1., 1.]], true)
    /// ] {
    ///     assert_eq!(d.unwrap().is_blts(), e);
    /// }
    /// ```
    pub fn is_blts(&self) -> bool {
        self.is_tor() && self.is_symmetrical() && self.is_uniform()
    }
}

/// Generates a PiecewiseLinearFunction from a qualitative domain of Trapezoidal labels.
///
/// # Examples
///
/// ```
/// # use assessment::fuzzy::membership::piecewise::PiecewiseLinearFunction;
/// # use assessment::qualitative_domain;
/// let domain_a = qualitative_domain![
///     "a" => vec![0.0, 0.0, 1.0],
///     "b" => vec![0.0, 1.0, 1.0]
/// ].unwrap();
/// let domain_b = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.5, 1.0, 1.0]
/// ].unwrap();
/// let domain_c = qualitative_domain![
///     "a" => vec![0.0, 0.0, 0.5],
///     "b" => vec![0.0, 0.5, 1.0],
///     "c" => vec![0.5, 1.0, 1.0]
/// ].unwrap();
///
/// assert_eq!(format!("{}", PiecewiseLinearFunction::from(&domain_a)), "([0.00, 1.00] => y = 0.00??x + 1.00)");
/// assert_eq!(format!("{}", PiecewiseLinearFunction::from(&domain_b)), "([0.00, 0.50] => y = -2.00??x + 1.00); ([0.50, 1.00] => y = 2.00??x - 1.00)");
/// assert_eq!(format!("{}", PiecewiseLinearFunction::from(&domain_c)), "([0.00, 1.00] => y = 0.00??x + 1.00)");
/// ```
impl From<&Qualitative<Trapezoidal>> for PiecewiseLinearFunction {
    fn from(domain: &Qualitative<Trapezoidal>) -> Self {
        let mut result = PiecewiseLinearFunction::new();
        domain
            .labels
            .iter()
            .map(PiecewiseLinearFunction::from)
            .for_each(|function| result = result.merge(&function));
        result
    }
}
