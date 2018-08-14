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
