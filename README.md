# Resolution Prover [![Travis CI Status](https://api.travis-ci.org/ExcaliburZero/resolution-prover.svg)](https://travis-ci.org/ExcaliburZero/resolution-prover) [![Coverage Status](https://coveralls.io/repos/github/ExcaliburZero/resolution-prover/badge.svg?branch=master)](https://coveralls.io/github/ExcaliburZero/resolution-prover?branch=master) [![Library documentation](https://img.shields.io/readthedocs/pip.svg)](https://excaliburzero.github.io/resolution-prover/master/resolution_prover/index.html)
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
* `(s \/ t) -> q`
* `t`

The prover converts the assumptions into conjunctive normal form such that they become clauses, statements that consist of multiple terms or'd with one another, that are connected by ands. The conversion is done by performing the following transformations.

* Converting all uses of conditionals (implications) and biconditionals (iff) into usages of disjunctions and negations. Using the fact that `(p -> q) :: (~p \/ q)` and `(p <-> q) :: ((p -> q) /\ (q -> p))`
* Reducing the scope of the negations in the statement by removing double negations and applying Demorgan's law such that the negations are propogated deeper into the statements. (`~~p :: p`, `~(p /\ q) :: ~p \/ ~q`, and `~(p \/ q) :: ~p /\ ~q`)
* Bubbling up the ands to the highest levels of scoping, through applications of distribution of ands and ors `(p \/ (q /\ r)) :: ((p \/ q) /\ (p \/ r))`.
* Splitting the statements on the ands into separate clauses, breaking up the resulting statements into their or'd parts to form the clauses.

In this case, we end up with the following clauses formed from the assumptions.

```
p               |  p
(p /\ q) -> r   |  ~p \/ ~q \/ r
(s \/ t) -> q   |  ~s \/ q
                |  ~t \/ q
t               |  t
```

Once all of the assumptions have been converted into conjuctive normal form and into clauses, the prover negates the goal to prepare for a proof by contradiction. The idea is to see if introducing the negation of the goal causes a contradiction, and if so then it follows that the goal is provable from the assumptions.

The prover will then convert the negated goal into conjunctive normal form and into clauses. Then it will select one of the formed clauses, adding the remaining negated goal clauses to the set of clauses formed by the assumptions.

The prover will then repeated select a clause from the list of clauses and try to resolve it against the "current" clause, by combining their terms and removing any pairs of terms that are negations of one another (ex `p` and `~p`). This process is done until the "current" clause becomes empty, and thus the goal is provable, or no remaing clauses are left to choose from, and thus the attempt is unsuccessful.

Backtracking is used to allow this process to work, taking into account bad choices for the negated goal clause to start with and the clauses chosen to resolve against the"current" clause. Additionally only clauses that could possibly resolve against the "current" clause are chosen, to prevent unecessary computation and backtracking.

For example, in this case the prover will negate our goal `r` to get `~r`. The negated goal is then converted into clauses, in this case the only clause is `~r`. The prover will then look for a clause that can resolve with `~r`. It chooses `~p \/ ~q \/ r`, since it is the only other clause with an `r` term. The two clauses areresolved to form `~p \/ ~q`.

The prover will then look for a clause that resolves with `~p`, finding `p` to resolve to `~q`. It will then look for a clause that resolves with `~q`, finding `~s \/ q` to resolve to `~s`. It will then look for a clause that resolves with `~s`, but be unable to find any, and thus begins to backtrack.

It will then look for another clause that resolves with `~q`, finding `~t \/ q` to resolve to `~t`. It will then look for a clause that resolves with `~t`, finding `t` to resolve to the empty clause. Since the empty clause was reached, a contradiction was found and thus the goal statement must be provable.

And indeed the following proof would yield the goal statement.

```
1) p
2) (p /\ q) -> r
3) (s \/ t) -> q
4) t
-----------------
5) s \/ t         Addition     4
6) q              Modus Ponens 3, 5
7) p /\ q         Conjunction  1, 6
8) r              Modus Ponens 2, 7
```

## Licensing
The source code is available under the [MIT License](https://opensource.org/licenses/MIT), see `LICENSE` for more information.
