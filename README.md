# Resolution Prover
This is a resolution prover library for propositional logic that I wrote in Rust after reading some of *Artifical Intelligence* by Elain Rich.

Given a set of assumptions and a proposition to prove using those assumptions, it will attempt a proof by contradiction by converting all of the propositions into conjuctive normal form and then resolving together clauses containing terms that are negations of one another.

```rust
use resolution_prover::*;

let assumptions = vec!(
    term("p".to_string()),
    implies(
        and(term("p".to_string()), term("q".to_string())),
        term("r".to_string())
    ),
    implies(
        or(term("s".to_string()), term("t".to_string())),
        term("q".to_string())
    ),
    term("t".to_string())
);

let goal = term("r".to_string());

assert_eq!(resolve(assumptions, goal), true);
```

In the example above (from *Artifical Intelligence*), we attempt to prove that `r` follows as true from the following set of assumptions:

* `p`
* `(p /\ q) -> r`
* `(s /\ t) -> q`
* `t`

The prover converts the assumptions into conjunctive normal form such that they become clauses, statements that consist of multiple terms or'd with one another, that are connected by ands. The conversion is done by performing the following transformations.

* Converting all uses of conditionals (implications) and biconditionals (iff) into usages of disjunctions and negations. Using the fact that `(p -> q) :: (~p \/ q)` and `(p <-> q) :: ((p -> q) /\ (q -> p))`
* Reducing the scope of the negations in the statement by removing double negations and applying Demorgan's law such that the negations are propogated deeper into the statements. (`~~p :: p`, `~(p /\ q) :: ~p \/ ~q`, and `~(p \/ q) :: ~p /\ ~q`)
* Bubbling up the ands to the highest levels of scoping, through applications of distribution of ands and ors `(p \/ (q /\ r)) :: ((p \/ q) /\ (p \/ r))`.
* Splitting the statements on the ands into separate clauses, breaking up the resulting statements into their or'd parts to form the clauses.

Once all of the assumptions have been converted into conjuctive normal form and into clauses, the prover negates the goal to prepare for a proof by contradiction. The idea is to see if introducing the negation of the goal causes a contradiction, and if so then it follows that the goal is provable from the assumptions.
