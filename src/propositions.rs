use std::fmt;

/// ```
/// let prop1 = resolution_prover::Proposition::Term("hello".to_string());
/// let prop2 = resolution_prover::Proposition::Term("hi".to_string());
///
/// let prop3 = resolution_prover::not(prop2);
///
/// let prop4 = resolution_prover::and(prop1, prop3);
/// ```
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Proposition {
    Or(Box<Proposition>, Box<Proposition>),
    And(Box<Proposition>, Box<Proposition>),
    Implies(Box<Proposition>, Box<Proposition>),
    Iff(Box<Proposition>, Box<Proposition>),
    Not(Box<Proposition>),
    Term(String)
}

pub fn or(a: Proposition, b: Proposition) -> Proposition {
    Proposition::Or(
        Box::new(a),
        Box::new(b)
    )
}

pub fn and(a: Proposition, b: Proposition) -> Proposition {
    Proposition::And(
        Box::new(a),
        Box::new(b)
    )
}

pub fn implies(a: Proposition, b: Proposition) -> Proposition {
    Proposition::Implies(
        Box::new(a),
        Box::new(b)
    )
}

pub fn iff(a: Proposition, b: Proposition) -> Proposition {
    Proposition::Iff(
        Box::new(a),
        Box::new(b)
    )
}

pub fn not(prop: Proposition) -> Proposition {
    Proposition::Not(Box::new(prop))
}

pub fn term(value: String) -> Proposition {
    Proposition::Term(value)
}

impl fmt::Display for Proposition {
    /// ```
    /// let prop1 = resolution_prover::Proposition::Term("hello".to_string());
    /// let prop2 = resolution_prover::Proposition::Term("hi".to_string());
    ///
    /// let prop3 = resolution_prover::not(prop2);
    ///
    /// let prop4 = resolution_prover::and(prop1, prop3);
    ///
    /// let expected = String::from("hello /\\ ~hi");
    ///
    /// assert_eq!(format!("{}", prop4), expected);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Proposition::Or(ref a, ref b) => {
                write!(f, "{} \\/ {}", a, b)
            },
            Proposition::And(ref a, ref b) => {
                write!(f, "{} /\\ {}", a, b)
            },
            Proposition::Implies(ref a, ref b) => {
                write!(f, "{} -> {}", a, b)
            },
            Proposition::Iff(ref a, ref b) => {
                write!(f, "{} <-> {}", a, b)
            },
            Proposition::Not(ref a) => write!(f, "~({})", a),
            Proposition::Term(ref a) => write!(f, "{}", a),
        }
    }
}
