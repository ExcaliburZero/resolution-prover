use propositions::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clause {
    pub parts: Vec<ClausePart>
}

impl Clause {
    /// ```
    /// /*let prop1 = resolution_prover::Proposition::Term("hello".to_string());
    ///
    /// let expected = resolution_prover::Clause {
    ///     parts: vec!(
    ///         resolution_prover::ClausePart::Term("hello".to_string())
    ///     )
    /// };
    ///
    /// assert_eq!(
    ///     resolution_prover::Clause::from_proposition(prop1),
    ///     expected
    /// );*/
    /// ```
    pub fn from_proposition(prop: Proposition) -> Clause {
        Clause {
            parts: Clause::break_into_clauses(prop)
        }
    }

    fn break_into_clauses(prop: Proposition) -> Vec<ClausePart> {
        let no_implication = Clause::eliminate_implication(prop);
        let red_negations = Clause::reduce_negation(no_implication);
        let or_not_prop = Clause::bubble_up_ands(red_negations);

        Clause::from_or_not_prop(or_not_prop)
    }

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
            Proposition::Or(a, b) => or(*a, *b),
            Proposition::And(a, b) => and(*a, *b),
            Proposition::Term(a) => term(a),
            p => panic!("Unexpected implies or iff: {}", p)
        }
    }

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

    fn from_or_not_prop(prop: Proposition) -> Vec<ClausePart> {
        match prop {
            Proposition::Or(a, b) => {
                let mut a_parts = Clause::from_or_not_prop(*a);
                let mut b_parts = Clause::from_or_not_prop(*b);

                a_parts.append(&mut b_parts);

                a_parts
            },
            Proposition::Not(inner) => {
                match *inner {
                    Proposition::Term(a) => vec!(ClausePart::NegatedTerm(a)),
                    _ => panic!("Proposition contained non-(or, not) term")
                }
            },
            Proposition::Term(a) => vec!(ClausePart::Term(a)),
            _ => panic!("Proposition contained non-(or, not) term: {}", prop)
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ClausePart {
    Term(String),
    NegatedTerm(String)
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
