use crate::fuzzy::membership::piecewise::PiecewiseLinearFunction;
use crate::fuzzy::membership::Membership;
use std::fmt::{Debug, Display, Formatter};

/// Label's membership trait alias
pub trait LabelMembership = Membership + Display;

/// Fuzzy label struct.
///
/// It is defined by a membership function and a name.
#[derive(Debug, PartialEq, Clone)]
pub struct Label<T: LabelMembership> {
    name: String,
    membership: T,
}

// Note: + Display added because clion doesn't detect here correctly the trait_alias feature
impl<T: LabelMembership + Display> Display for Label<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.name, self.membership)
    }
}

/// Label error types.
#[derive(Debug, PartialEq)]
pub enum LabelError {
    /// Non standardized name (see [standardize_name]).
    NonStandardizedName { name: String },
    /// Empty name.
    EmptyName,
}

impl Display for LabelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use LabelError::*;
        match &self {
            NonStandardizedName { name } => {
                write!(f, "Name '{}' isn't standardized.", name)
            }
            EmptyName => {
                write!(f, "Empty name provided.")
            }
        }
    }
}

impl<T: LabelMembership> Label<T> {
    /// Creates a new label.
    ///
    /// # Arguments
    /// * `name`: Label name.
    /// * `membership`: Membership function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::{Label, membership::Trapezoidal};
    /// let label = Label::new(
    ///     String::from("a"),
    ///     Trapezoidal::new(vec![0.0, 0.5, 1.0]).unwrap()
    /// ).unwrap();
    /// assert_eq!(format!("{}", label), "a => (0.00, 0.50, 1.00)");
    /// ```
    ///
    /// # Errors
    ///
    /// **LabelError::NonStandardizedName**: If `name` isn't standardized.
    ///
    /// ```
    /// # use assessment::fuzzy::{Label, LabelError, membership::Trapezoidal};
    /// let names = vec![" a", "A", " c "];
    /// for name in names {
    ///     assert_eq!(
    ///         Label::new(
    ///             name.to_string(),
    ///             Trapezoidal::new(vec![0.0, 0.5, 1.0]).unwrap()
    ///         ),
    ///         Err(LabelError::NonStandardizedName { name: name.to_string() })
    ///     );
    /// }
    /// ```
    ///
    /// **LabelError::EmptyName**: If `name.len() == 0`.
    ///
    /// ```
    /// # use assessment::fuzzy::{Label, LabelError, membership::Trapezoidal};
    /// assert_eq!(
    ///     Label::new(
    ///         String::new(),
    ///         Trapezoidal::new(vec![0.0, 0.5, 1.0]).unwrap()
    ///     ),
    ///     Err(LabelError::EmptyName)
    /// );
    /// ```
    ///
    pub fn new(name: String, membership: T) -> Result<Self, LabelError> {
        use LabelError::*;
        if !is_standardized(&name) {
            Err(NonStandardizedName { name })
        } else if name.is_empty() {
            Err(EmptyName)
        } else {
            Ok(Self { name, membership })
        }
    }

    /// Returns label name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::{Label, membership::Trapezoidal};
    /// let name = String::from("a");
    /// let label = Label::new(
    ///     name.clone(),
    ///     Trapezoidal::new(vec![0.0, 0.5, 1.0]).unwrap()
    /// ).unwrap();
    /// assert_eq!(*label.name(), name);
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns label membership.
    ///
    /// # Examples
    ///
    /// ```
    /// # use assessment::fuzzy::{Label, membership::Trapezoidal};
    /// let membership = Trapezoidal::new(vec![0.0, 0.5, 1.0]).unwrap();
    /// let label = Label::new(
    ///     String::from("a"),
    ///     membership
    /// ).unwrap();
    /// assert_eq!(format!("{}", *label.membership()), "(0.00, 0.50, 1.00)");
    /// ```
    pub fn membership(&self) -> &T {
        &self.membership
    }
}

/// Standardizes a name.
///
/// A name is standardized if `name == name.trim().to_lowercase().
///
/// # Arguments
/// * `name`: A string slice.
///
/// # Examples
///
/// ```
/// # use assessment::fuzzy::label::standardize_name;
/// for (v, e) in [
///     ("ok", "ok"),
///     (" NoT oK ", "not ok")
/// ] {
///     assert_eq!(standardize_name(v), e);
/// }
/// ```
pub fn standardize_name(name: &str) -> String {
    name.trim().to_lowercase()
}

/// Checks is a name is standardized.
///
/// # Arguments
/// * `name`: A string slice.
///
/// # Examples
///
/// ```
/// # use assessment::fuzzy::label::is_standardized;
/// for (v, e) in [
///     ("ok", true),
///     (" NoT oK ", false)
/// ] {
///     assert_eq!(is_standardized(v), e);
/// }
/// ```
pub fn is_standardized(name: &str) -> bool {
    name.to_string() == standardize_name(name)
}

/// Returns labels names.
///
/// # Arguments
/// * `labels`: A vector of labels.
///
/// # Examples
///
/// ```
/// # use assessment::fuzzy::label::get_labels_names;
/// # use assessment::trapezoidal_labels;
/// let labels = trapezoidal_labels![
///     "a" => vec![0.0, 0.0, 1.0],
///     "b" => vec![0.0, 1.0, 1.0]
/// ].unwrap();
/// assert_eq!(get_labels_names(&labels), vec!["a", "b"]);
/// ```
pub fn get_labels_names<T: Display + LabelMembership>(labels: &Vec<Label<T>>) -> Vec<&str> {
    labels
        .iter()
        .map(|l| l.name().as_str())
        .collect::<Vec<&str>>()
}

#[allow(unused_imports)]
use crate::fuzzy::membership::Trapezoidal;
/// Trapezoidal labels.
///
/// Generates an array of trapezoidal labels.
///
/// # Examples
///
/// ```
/// # use assessment::trapezoidal_labels;
/// let labels = trapezoidal_labels![
///     "a" => vec![0.0, 0.0, 1.0],
///     "b" => vec![0.0, 1.0, 1.0]
/// ].unwrap();
///
/// assert_eq!(labels.len(), 2);
/// assert_eq!(format!("{}", labels[0]), "a => (0.00, 0.00, 1.00)");
/// assert_eq!(format!("{}", labels[1]), "b => (0.00, 1.00, 1.00)");
/// ```
///
/// ```
/// # use assessment::trapezoidal_labels;
/// let labels = trapezoidal_labels![].unwrap();
/// assert_eq!(labels.len(), 0);
/// ```
///
/// # Errors
///
/// **String**: If any label name is invalid (see [Label::new]).
///
/// ```
/// # use assessment::trapezoidal_labels;
/// assert!(
///     trapezoidal_labels![
///         " a" => vec![0.0, 0.0, 1.0, 1.0, 1.0]
///     ].is_err()
/// );
/// ```
///
/// **String**: If any label limits are invalid (see [Trapezoidal::new])
///
/// ```
/// # use assessment::trapezoidal_labels;
/// assert!(
///     trapezoidal_labels![
///         "a" => vec![0.0, 0.0, 1.0, 1.0, 1.0]
///     ].is_err()
/// );
/// ```
///
///
#[macro_export]
macro_rules! trapezoidal_labels {
    ( $( $name:expr => $membership:expr ),* ) => {
        {
            let mut labels = Vec::<$crate::fuzzy::Label<$crate::fuzzy::membership::Trapezoidal>>::new();
            let mut abort = false;
            let mut error = String::new();
            $(
                match abort {
                    false => {
                        match $crate::fuzzy::membership::Trapezoidal::new($membership) {
                            Ok(t) => {
                                match $crate::fuzzy::Label::new($name.to_string(), t) {
                                    Ok(l) => labels.push(l),
                                    Err(e) => {
                                        error = format!("{}", e);
                                        abort = true;
                                    }
                                }
                            },
                            Err(e) => {
                                error = format!("{}", e);
                                abort = true;
                            }
                        }
                    },
                    _ => (),
                }
            )*

            if abort {
                Err(error)
            } else {
                Ok(labels)
            }
        }
    };
}

/// Generates a PiecewiseLinearFunction from a trapezoidal label
///
/// # Examples
///
/// ```
/// # use assessment::trapezoidal_labels;
/// # use assessment::fuzzy::membership::piecewise::PiecewiseLinearFunction;
/// let labels = trapezoidal_labels![
///     "a" => vec![0.0, 0.1, 0.2, 0.3],
///     "b" => vec![0.0, 0.1, 0.1, 0.2],
///     "c" => vec![0.0, 0.1, 0.2]
/// ].unwrap();
///
/// assert_eq!(format!("{}", PiecewiseLinearFunction::from(&labels[0])), "([0.00, 0.10] => y = 10.00·x + 0.00); ([0.10, 0.20] => y = 0.00·x + 1.00); ([0.20, 0.30] => y = -10.00·x + 3.00)");
/// assert_eq!(format!("{}", PiecewiseLinearFunction::from(&labels[1])), "([0.00, 0.10] => y = 10.00·x + 0.00); ([0.10, 0.20] => y = -10.00·x + 2.00)");
/// assert_eq!(format!("{}", PiecewiseLinearFunction::from(&labels[2])), "([0.00, 0.10] => y = 10.00·x + 0.00); ([0.10, 0.20] => y = -10.00·x + 2.00)");
/// ```
impl From<&Label<Trapezoidal>> for PiecewiseLinearFunction {
    fn from(l: &Label<Trapezoidal>) -> Self {
        PiecewiseLinearFunction::from(&l.membership)
    }
}
