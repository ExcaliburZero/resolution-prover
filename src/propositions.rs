use std::fmt;

/// A statement in propositional logic.
///
/// For example, `p /\ (~q)` would be representated in the following way.
///
/// ```
/// let p = resolution_prover::term("p".to_string());
/// let q = resolution_prover::term("q".to_string());
///
/// let not_q = resolution_prover::not(q);
///
/// let p_and_not_q = resolution_prover::and(p, not_q);
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

/// Creates a proposition that is the disjunction of the two given
/// propositions.
///
/// ```
/// let p = resolution_prover::term("p".to_string());
/// let q = resolution_prover::term("q".to_string());
///
/// let p_or_q = resolution_prover::or(p, q);
///
/// let expected = "p \\/ q";
///
/// assert_eq!(p_or_q.to_string(), expected);
/// ```
pub fn or(a: Proposition, b: Proposition) -> Proposition {
    Proposition::Or(
        Box::new(a),
        Box::new(b)
    )
}

/// Creates a proposition that is the conjucntion of the two given
/// propositions.
///
/// ```
/// let p = resolution_prover::term("p".to_string());
/// let q = resolution_prover::term("q".to_string());
///
/// let p_and_q = resolution_prover::and(p, q);
///
/// let expected = "p /\\ q";
///
/// assert_eq!(p_and_q.to_string(), expected);
/// ```
pub fn and(a: Proposition, b: Proposition) -> Proposition {
    Proposition::And(
        Box::new(a),
        Box::new(b)
    )
}

/// Creates a proposition that is the implication consisting of the two given
/// propositions as the antecedent and consequent repsectively.
///
/// ```
/// let p = resolution_prover::term("p".to_string());
/// let q = resolution_prover::term("q".to_string());
///
/// let p_implies_q = resolution_prover::implies(p, q);
///
/// let expected = "p -> q";
///
/// assert_eq!(p_implies_q.to_string(), expected);
/// ```
pub fn implies(a: Proposition, b: Proposition) -> Proposition {
    Proposition::Implies(
        Box::new(a),
        Box::new(b)
    )
}

/// Creates a proposition that is the biconditional of the two given
/// propositions.
///
/// ```
/// let p = resolution_prover::term("p".to_string());
/// let q = resolution_prover::term("q".to_string());
///
/// let p_iff_q = resolution_prover::iff(p, q);
///
/// let expected = "p <-> q";
///
/// assert_eq!(p_iff_q.to_string(), expected);
/// ```
pub fn iff(a: Proposition, b: Proposition) -> Proposition {
    Proposition::Iff(
        Box::new(a),
        Box::new(b)
    )
}

/// Creates a proposition that is the negation of the given proposition.
///
/// ```
/// let p = resolution_prover::term("p".to_string());
///
/// let not_p = resolution_prover::not(p);
///
/// let expected = "~(p)";
///
/// assert_eq!(not_p.to_string(), expected);
/// ```
pub fn not(prop: Proposition) -> Proposition {
    Proposition::Not(Box::new(prop))
}

/// Creates a term from the given string.
///
/// ```
/// let p = resolution_prover::term("p".to_string());
///
/// let expected = "p";
///
/// assert_eq!(p.to_string(), expected);
/// ```
pub fn term(value: String) -> Proposition {
    Proposition::Term(value)
}

impl fmt::Display for Proposition {
    /// Displays the proposition using cominations of ascii characters to
    /// represent the propositional logic operations.
    ///
    /// ```
    /// let hello = resolution_prover::Proposition::Term("hello".to_string());
    /// let hi = resolution_prover::Proposition::Term("hi".to_string());
    ///
    /// let not_hi = resolution_prover::not(hi);
    ///
    /// let hello_and_not_hi = resolution_prover::and(hello, not_hi);
    ///
    /// let expected = String::from("hello /\\ ~(hi)");
    ///
    /// assert_eq!(format!("{}", hello_and_not_hi), expected);
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
