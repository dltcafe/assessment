use crate::fuzzy::membership;
use std::fmt::{Display, Formatter};

/// Fuzzy label struct
///
/// It is defined by a membership function and a name.
#[derive(Debug)]
pub struct Label {
    name: String,
    membership: SupportedMembership,
}

// // //
// Enums
//

/// Supported membership functions
#[derive(Debug)]
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
    pub fn new(name: String, membership: SupportedMembership) -> Self {
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
