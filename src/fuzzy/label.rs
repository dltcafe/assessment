use crate::fuzzy::membership;
use std::fmt::{Display, Formatter};

/// Fuzzy label struct
///
/// It is defined by a membership function and a name.
#[derive(Debug, PartialEq, Clone)]
pub struct Label {
    name: String,
    membership: SupportedMembership,
}

// // //
// Enums
//

/// Supported membership functions
#[derive(Debug, PartialEq, Clone)]
pub enum SupportedMembership {
    Trapezoidal(membership::Trapezoidal),
}

impl Display for SupportedMembership {
    fn fmt(&self, mut f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            SupportedMembership::Trapezoidal(membership) => membership.fmt(&mut f),
        }
    }
}

// // //
// Traits implementations
//

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.name, self.membership)
    }
}

// // //
// Implementations
//

impl Label {
    /// Force `name::trim().to_lowercase() == name` and `name.length > 0`
    fn _force_valid_label_name(name: &str) {
        if name.trim().to_lowercase() != name {
            panic!(
                "Name without spaces and to lowercase should be equals to itself, provided \"{}\"",
                name
            );
        }

        if name.is_empty() {
            panic!("Name is empty");
        }
    }

    /// Creates a new label
    ///
    /// # Params
    /// - `name`: Label name.
    /// - `membership`: Membership function.
    ///
    /// # Examples
    ///
    /// ```
    /// use assessment::fuzzy::label::*;
    /// use assessment::fuzzy::membership::Trapezoidal;
    ///
    /// let name = String::from("a");
    /// let membership = SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.5, 1.0]));
    /// let label = Label::new(name, membership);
    /// assert_eq!(format!("{}", label), "a => (0.00, 0.50, 1.00)");
    /// ```
    ///
    /// # Panics
    ///
    /// If `name.trim().len() != name.len()`
    ///
    /// ```should_panic
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let name = String::from(" a");
    /// let membership = SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.5, 1.0]));
    /// let label = Label::new(name, membership);
    /// ```
    ///
    /// If `name.to_lowercase() != name`
    ///
    /// ```should_panic
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let name = String::from("A");
    /// let membership = SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.5, 1.0]));
    /// let label = Label::new(name, membership);
    /// ```
    ///
    /// If `name.len() == 0`
    ///
    /// ```should_panic
    /// use assessment::fuzzy::{label::*, membership::Trapezoidal};
    ///
    /// let name = String::from("");
    /// let membership = SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.5, 1.0]));
    /// let label = Label::new(name, membership);
    /// ```
    ///
    pub fn new(name: String, membership: SupportedMembership) -> Self {
        Label::_force_valid_label_name(&name);
        Self { name, membership }
    }

    /// Returns label name
    ///
    /// # Examples
    ///
    /// ```
    /// use assessment::fuzzy::label::*;
    /// use assessment::fuzzy::membership::Trapezoidal;
    ///
    /// let name = String::from("a");
    /// let aux = name.clone();
    /// let membership = SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.5, 1.0]));
    /// let label = Label::new(name, membership);
    ///
    /// assert_eq!(*label.name(), aux);
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns label name
    ///
    /// # Examples
    ///
    /// ```
    /// use assessment::fuzzy::label::*;
    /// use assessment::fuzzy::membership::Trapezoidal;
    ///
    /// let name = String::from("a");
    /// let membership = SupportedMembership::Trapezoidal(Trapezoidal::new(&vec![0.0, 0.5, 1.0]));
    /// let label = Label::new(name, membership);
    ///
    /// assert_eq!(format!("{}", *label.membership()), "(0.00, 0.50, 1.00)");
    /// ```
    pub fn membership(&self) -> &SupportedMembership {
        &self.membership
    }
}

// // //
// Macros
//

/// Trapezoidal labels
///
/// Generates an array of trapezoidal labels
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
/// assert_eq!(labels.len(), 2);
/// assert_eq!(format!("{}", labels[0]), "a => (0.00, 0.00, 1.00)");
/// assert_eq!(format!("{}", labels[1]), "b => (0.00, 1.00, 1.00)");
/// ```
///
/// # Panics
///
/// If any label name is invalid (see Label::new(&self, name))
///
/// ```should_panic
/// use assessment::trapezoidal_labels;
///
/// let mut labels = trapezoidal_labels![
///     " a" => &vec![0.0, 0.0, 1.0]
/// ];
/// ```
///
/// If any label limits are invalid (see Trapezoidal::new(&self, &limits))
///
/// ```should_panic
/// use assessment::trapezoidal_labels;
///
/// let mut labels = trapezoidal_labels![
///     "a" => &vec![0.0, 0.0, 1.0, 1.0, 1.0]
/// ];
/// ```
#[macro_export]
macro_rules! trapezoidal_labels {
    ( $( $name:expr => $membership:expr ),* ) => {
        {
            let mut labels = Vec::new();
            $(
                labels.push(
                    assessment::fuzzy::label::Label::new(
                        $name.to_string(),
                        assessment::fuzzy::label::SupportedMembership::Trapezoidal(
                            assessment::fuzzy::membership::Trapezoidal::new($membership)
                        )
                    )
                );
            )*
            labels
        }
    };
}
