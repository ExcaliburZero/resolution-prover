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
        let or_not_prop = Clause::simplify_prop(prop);

        Clause::from_or_not_prop(or_not_prop)
    }

    fn simplify_prop(prop: Proposition) -> Proposition {
        match prop {
            Proposition::Implies(a, b) => {
                let a_simpl = Clause::simplify_prop(*a);
                let b_simpl = Clause::simplify_prop(*b);
                or(not(a_simpl), b_simpl)
            },
            Proposition::Term(a) => term(a),
            _ => panic!("Ahh! ...")
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
    fn simplify_prop_case1() {
        let prop = implies(
            term("a".to_string()),
            term("b".to_string())
        );

        let expected = or(
            not(term("a".to_string())),
            term("b".to_string())
        );

        assert_eq!(Clause::simplify_prop(prop), expected);
    }
}
