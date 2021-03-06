use propositions::*;

/// A proposition made up of the disjunction of possibly negated terms.
///
/// For example the proposition `p \/ ~q \/ r` would be represetnted as the
/// following.
///
/// ```
/// let clause = resolution_prover::Clause {
///     parts: vec!(
///         resolution_prover::ClausePart::Term("p".to_string()),
///         resolution_prover::ClausePart::NegatedTerm("q".to_string()),
///         resolution_prover::ClausePart::Term("r".to_string())
///     )
/// };
/// ```
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
pub struct Clause {
    pub parts: Vec<ClausePart>
}

impl Clause {
    /// Converts the given proposition into the corresponding clauses. Note
    /// that one proposition could break down into one or more clauses.
    ///
    /// ```
    /// let prop1 = resolution_prover::term("hello".to_string());
    ///
    /// let expected = vec!(resolution_prover::Clause {
    ///     parts: vec!(
    ///         resolution_prover::ClausePart::Term("hello".to_string())
    ///     )
    /// });
    ///
    /// assert_eq!(
    ///     resolution_prover::Clause::from_proposition(prop1),
    ///     expected
    /// );
    /// ```
    pub fn from_proposition(prop: Proposition) -> Vec<Clause> {
        let all_parts = Clause::break_into_clauses(prop);

        all_parts.iter()
            .map(|parts| Clause { parts: parts.to_vec() })
            .collect()
    }

    /// Breaks down the given proposition into the parts of the equivalent
    /// clauses.
    ///
    /// In the returned value, the first level of `Vec` represents the
    /// different clauses, and the second level of `Vec` represents the parts
    /// of that specific clause.
    fn break_into_clauses(prop: Proposition) -> Vec<Vec<ClausePart>> {
        let no_implication = Clause::eliminate_implication(prop);
        let red_negations = Clause::reduce_negation(no_implication);
        let bubbled = Clause::bubble_up_ands(red_negations);
        let or_not_props = Clause::split_on_ands(bubbled);

        or_not_props.iter()
            .map(|p| Clause::from_or_not_prop(p))
            .collect()
    }

    /// Converts the given proposition to an equivalent proposition that does
    /// not use any instances of implication or biconditional.
    ///
    /// The conversions are done by breaking biconditionals into the and of
    /// the implication in each direction, and then converting the implications
    /// into the disjunction of the negation of the antecedent and the
    /// consequent.
    fn eliminate_implication(prop: Proposition) -> Proposition {
        match prop {
            Proposition::Implies(a, b) => {
                let a_simpl = Clause::eliminate_implication(*a);
                let b_simpl = Clause::eliminate_implication(*b);
                or(not(a_simpl), b_simpl)
            },
            Proposition::Iff(a, b) => {
                and(
                    Clause::eliminate_implication(implies(*a.clone(), *b.clone())),
                    Clause::eliminate_implication(implies(*b, *a))
                )
            }
            p => p
        }
    }

    /// Reduces the score of the negation in the given proposition, moving the
    /// negation inwards as far as possible.
    ///
    /// The conversion is done by eliminating double negations and using
    /// deMorgan's law.
    ///
    /// # Panics
    ///
    /// This function assumes that all implications and biconditionals have
    /// already been removed from the proposition, and will panic upon finding
    /// any.
    fn reduce_negation(prop: Proposition) -> Proposition {
        match prop {
            Proposition::Not(a) => {
                let a2 = *a; // Pull the value out to allow multiple matching
                match a2 {
                    Proposition::Not(b) => Clause::reduce_negation(*b),
                    Proposition::And(b, c) => or(
                        Clause::reduce_negation(not(*b)),
                        Clause::reduce_negation(not(*c))
                    ),
                    Proposition::Or(b, c) => and(
                        Clause::reduce_negation(not(*b)),
                        Clause::reduce_negation(not(*c))
                    ),
                    Proposition::Term(b) => not(term(b)),
                    p => panic!("Unexpected implies or iff: {}", not(p))
                }
            },
            Proposition::Or(a, b) => or(
                Clause::reduce_negation(*a),
                Clause::reduce_negation(*b)
            ),
            Proposition::And(a, b) => and(
                Clause::reduce_negation(*a),
                Clause::reduce_negation(*b)
            ),
            Proposition::Term(a) => term(a),
            p => panic!("Unexpected implies or iff: {}", p)
        }
    }

    /// Bubbles up the conjunctions in the given proposition so that the
    /// proposition becomes the conjunctions of terms and disjunctions.
    ///
    /// This conversion is done by using the distributed property of
    /// conujunctions and disjunctions.
    fn bubble_up_ands(prop: Proposition) -> Proposition {
        match Clause::bubble_up_ands_(prop, false) {
            (p, _) => p
        }
    }

    fn bubble_up_ands_(prop: Proposition, cleared: bool) -> (Proposition, bool) {
        let handled = match (prop, cleared) {
            (p, true) => p,
            (Proposition::Term(a), _) => term(a),
            (Proposition::Not(a), _) =>
                not(Clause::bubble_up_ands_(*a, false).0),
            (Proposition::Or(a, b), _) => {
                let a_bubbled = Clause::bubble_up_ands_(*a, false).0;
                let b_bubbled = Clause::bubble_up_ands_(*b, false).0;

                or(a_bubbled, b_bubbled)
            },
            (Proposition::And(a, b), _) => {
                let a_bubbled = Clause::bubble_up_ands_(*a, false).0;
                let b_bubbled = Clause::bubble_up_ands_(*b, false).0;

                and(a_bubbled, b_bubbled)
            },
            (p, _) => Clause::bubble_up_ands_(p, false).0,
        };

        match handled {
            Proposition::Or(a, b) => {
                let a2 = *a;
                let b2 = *b;
                match (a2, b2) {
                    (Proposition::And(c, d), e) =>
                        (and(
                            or(*c, e.clone()),
                            or(*d, e)
                        ), true),
                    (c, Proposition::And(d, e)) =>
                        (and(
                            or(c.clone(), *d),
                            or(c, *e)
                        ), true),
                    (p1, p2) => (or(p1, p2), true)
                }
            },
            Proposition::And(a, b) => (and(*a, *b), true),
            Proposition::Not(a) => (not(*a), true),
            Proposition::Term(a) => (term(a), true),
            p => panic!("Unexpected implies or iff: {}", p)
        }
    }

    /// Splits the given proposition on its conjunctions.
    ///
    /// Assumes that the proposition has already had its conjunctions bubbled
    /// up.
    fn split_on_ands(prop: Proposition) -> Vec<Proposition> {
        match prop {
            Proposition::And(a, b) => {
                let mut a_parts = Clause::split_on_ands(*a);
                let mut b_parts = Clause::split_on_ands(*b);

                a_parts.append(&mut b_parts);

                a_parts
            },
            p => vec!(p)
        }
    }

    /// Converts the given proposition into a set of clause parts by splitting
    /// it on the disjunctions.
    ///
    /// Assumes that the proposition has been simplified to contain only
    /// disjunctions, negations, and raw terms.
    fn from_or_not_prop(prop: &Proposition) -> Vec<ClausePart> {
        match *prop {
            Proposition::Or(ref a, ref b) => {
                let mut a_parts = Clause::from_or_not_prop(&*a);
                let mut b_parts = Clause::from_or_not_prop(&*b);

                a_parts.append(&mut b_parts);

                a_parts
            },
            Proposition::Not(ref inner) => {
                match **inner {
                    Proposition::Term(ref a) =>
                        vec!(ClausePart::NegatedTerm(a.clone())),
                    _ => panic!("Proposition contained non-(or, not) term: {}", prop)
                }
            },
            Proposition::Term(ref a) => vec!(ClausePart::Term(a.clone())),
            _ => panic!("Proposition contained non-(or, not) term: {}", prop)
        }
    }
}

/// A term or negated term that is part of a clause.
///
/// For example, `p` and `~q` can be represented as the following.
///
/// ```
/// let p = resolution_prover::ClausePart::Term("p".to_string());
///
/// let not_q = resolution_prover::ClausePart::NegatedTerm("q".to_string());
/// ```
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
pub enum ClausePart {
    Term(String),
    NegatedTerm(String)
}

impl ClausePart {
    /// Returns the negated version of the clause part.
    ///
    /// ```
    /// let clausePart = resolution_prover::ClausePart::Term("p".to_string());
    ///
    /// let neg = clausePart.negate();
    ///
    /// let expected_1 =
    ///     resolution_prover::ClausePart::NegatedTerm("p".to_string());
    ///
    /// assert_eq!(neg, expected_1);
    ///
    /// let double_neg = neg.negate();
    ///
    /// assert_eq!(double_neg, clausePart);
    /// ```
    pub fn negate(&self) -> ClausePart {
        match self {
            ClausePart::Term(a) => ClausePart::NegatedTerm(a.clone()),
            ClausePart::NegatedTerm(a) => ClausePart::Term(a.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use propositions::*;
    use clauses::*;

    #[test]
    fn eliminate_implication_implies() {
        let prop = implies(
            term("a".to_string()),
            term("b".to_string())
        );

        let expected = or(
            not(term("a".to_string())),
            term("b".to_string())
        );

        assert_eq!(Clause::eliminate_implication(prop), expected);
    }

    #[test]
    fn eliminate_implication_iff() {
        let prop = iff(
            term("a".to_string()),
            term("b".to_string())
        );

        let expected = and(
            or(
                not(term("a".to_string())),
                term("b".to_string())
            ),
            or(
                not(term("b".to_string())),
                term("a".to_string())
            )
        );

        assert_eq!(Clause::eliminate_implication(prop), expected);
    }

    #[test]
    fn eliminate_implication_and() {
        let prop = and(
            term("a".to_string()),
            term("b".to_string())
        );

        let expected = and(
            term("a".to_string()),
            term("b".to_string())
        );

        assert_eq!(Clause::eliminate_implication(prop), expected);
    }

    #[test]
    fn eliminate_implication_or() {
        let prop = or(
            term("a".to_string()),
            term("b".to_string())
        );

        let expected = or(
            term("a".to_string()),
            term("b".to_string())
        );

        assert_eq!(Clause::eliminate_implication(prop), expected);
    }

    #[test]
    fn eliminate_implication_not() {
        let prop = not(
            term("b".to_string())
        );

        let expected = not(
            term("b".to_string())
        );

        assert_eq!(Clause::eliminate_implication(prop), expected);
    }

    #[test]
    fn reduce_negation_term() {
        let prop = term("b".to_string());

        let expected = term("b".to_string());

        assert_eq!(Clause::reduce_negation(prop), expected);
    }

    #[test]
    fn reduce_negation_double_negation() {
        let prop = not(not(
            term("b".to_string())
        ));

        let expected = term("b".to_string());

        assert_eq!(Clause::reduce_negation(prop), expected);
    }

    #[test]
    fn reduce_negation_not_and() {
        let prop = not(and(
            term("a".to_string()),
            term("b".to_string())
        ));

        let expected = or(
            not(term("a".to_string())),
            not(term("b".to_string()))
        );

        assert_eq!(Clause::reduce_negation(prop), expected);
    }

    #[test]
    fn reduce_negation_not_or() {
        let prop = not(or(
            term("a".to_string()),
            term("b".to_string())
        ));

        let expected = and(
            not(term("a".to_string())),
            not(term("b".to_string()))
        );

        assert_eq!(Clause::reduce_negation(prop), expected);
    }

    #[test]
    fn reduce_negation_nested_not_or_double_negation() {
        let prop = not(or(
            term("a".to_string()),
            not(not(term("b".to_string())))
        ));

        let expected = and(
            not(term("a".to_string())),
            not(term("b".to_string()))
        );

        assert_eq!(Clause::reduce_negation(prop), expected);
    }

    #[test]
    fn reduce_negation_nested_not_or_or() {
        let prop = not(or(
            term("a".to_string()),
            or(
                term("b".to_string()),
                not(term("c".to_string())),
            )
        ));

        let expected = and(
            not(term("a".to_string())),
            and(
                not(term("b".to_string())),
                term("c".to_string())
            ),
        );

        assert_eq!(Clause::reduce_negation(prop), expected);
    }

    #[test]
    fn reduce_negation_nested_and_not_and() {
        let prop = and(
            not(and(
                term("a".to_string()),
                term("b".to_string())
            )),
            term("c".to_string())
        );

        let expected = and(
            or(
                not(term("a".to_string())),
                not(term("b".to_string()))
            ),
            term("c".to_string())
        );


        assert_eq!(Clause::reduce_negation(prop), expected);
    }

    #[test]
    fn reduce_negation_nested_or_not_and() {
        let prop = or(
            not(and(
                term("a".to_string()),
                term("b".to_string())
            )),
            term("c".to_string())
        );

        let expected = or(
            or(
                not(term("a".to_string())),
                not(term("b".to_string()))
            ),
            term("c".to_string())
        );


        assert_eq!(Clause::reduce_negation(prop), expected);
    }

    #[test]
    fn bubble_up_ands_term() {
        let prop = term("a".to_string());

        let expected = term("a".to_string());

        assert_eq!(Clause::bubble_up_ands(prop), expected);
    }

    #[test]
    fn bubble_up_ands_or() {
        let prop = or(
            term("a".to_string()),
            term("b".to_string())
        );

        let expected = or(
            term("a".to_string()),
            term("b".to_string())
        );

        assert_eq!(Clause::bubble_up_ands(prop), expected);
    }

    #[test]
    fn bubble_up_ands_or_and_left() {
        let prop = or(
            and(
                term("a".to_string()),
                term("b".to_string())
            ),
            term("c".to_string()),
        );

        let expected = and(
            or(
                term("a".to_string()),
                term("c".to_string())
            ),
            or(
                term("b".to_string()),
                term("c".to_string())
            ),
        );

        assert_eq!(Clause::bubble_up_ands(prop), expected);
    }

    #[test]
    fn bubble_up_ands_or_and_right() {
        let prop = or(
            term("a".to_string()),
            and(
                term("b".to_string()),
                term("c".to_string())
            )
        );

        let expected = and(
            or(
                term("a".to_string()),
                term("b".to_string())
            ),
            or(
                term("a".to_string()),
                term("c".to_string())
            ),
        );

        assert_eq!(Clause::bubble_up_ands(prop), expected);
    }

    #[test]
    fn bubble_up_ands_not() {
        let prop = not(term("a".to_string()));

        let expected = not(term("a".to_string()));

        assert_eq!(Clause::bubble_up_ands(prop), expected);
    }
}
