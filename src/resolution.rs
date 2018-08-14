extern crate multimap;

use clauses::*;
use propositions::*;

use self::multimap::MultiMap;

use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub fn resolve(assumptions: Vec<Proposition>, goal: Proposition) -> bool {
    let assumption_clauses: Vec<Clause> = assumptions.iter()
        .flat_map(|a| Clause::from_proposition(a.clone()))
        .collect();

    let mut clauses = ClauseStorage::new();
    assumption_clauses.into_iter()
        .for_each(|c| clauses.put(c));

    let negated_goal = not(goal);
    let neg_goal_clauses = Clause::from_proposition(negated_goal);

    for c in neg_goal_clauses {
        let mut visited = HashSet::new();
        visited.insert(&c);

        if resolve_(&clauses, &c, visited) {
            return true
        }
    }
    false
}

fn resolve_(clauses: &ClauseStorage, current: &Clause, visited: HashSet<&Clause>) -> bool {
    for p in current.parts.clone() {
        let matches = clauses.get(&p.negate(), &visited);
        for m in matches {
            let next = combine(current, m);

            if next.parts.len() == 0 {
                return true
            }

            let mut new_visited = visited.clone();
            new_visited.insert(&next);

            if resolve_(clauses, &next, new_visited) {
                return true
            }
        }
    }
    false
}

fn combine(a: &Clause, b: &Clause) -> Clause {
    let mut all_parts = HashSet::new();

    for p in a.parts.clone() {
        all_parts.insert(p);
    }
    for p in b.parts.clone() {
        all_parts.insert(p);
    }

    for ap in a.parts.clone() {
        for bp in b.parts.clone() {
            if ap.negate() == bp {
                all_parts.remove(&ap);
                all_parts.remove(&bp);
            }
        }
    }

    Clause {
        parts: all_parts.iter()
            .map(|p| p.clone())
            .collect()
    }
}

#[derive(Debug)]
struct ClauseStorage {
    lookup_table: MultiMap<ClausePart, usize>,
    clauses: Vec<Clause>
}

impl ClauseStorage {
    fn new() -> ClauseStorage {
        ClauseStorage {
            lookup_table: MultiMap::new(),
            clauses: vec!(),
        }
    }

    fn get(&self, part: &ClausePart, visited: &HashSet<&Clause>) -> Vec<&Clause> {
        match self.lookup_table.get_vec(part) {
            Some(indices) => {
                indices.iter()
                    .map(|i| &self.clauses[*i])
                    .filter(|v| !visited.contains(v))
                    .collect()
            }
            None => vec!()
        }
    }

    fn put(&mut self, clause: Clause) {
        let index = self.clauses.len();

        clause.parts.iter()
            .for_each(|p| self.lookup_table.insert((*p).clone(), index));

        self.clauses.push(clause);
    }
}

#[cfg(test)]
mod tests {
    use clauses::*;
    use propositions::*;
    use resolution::*;

    #[test]
    fn resolve_trivial_false() {
        let assumptions = vec!(
            term("a".to_string())
        );

        let goal = not(term("a".to_string()));

        assert_eq!(resolve(assumptions, goal), false);
    }

    #[test]
    fn resolve_trivial_true() {
        let assumptions = vec!(
            term("a".to_string())
        );

        let goal = term("a".to_string());

        assert_eq!(resolve(assumptions, goal), true);
    }

    #[test]
    fn resolve_simple_true() {
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
    }

    #[test]
    fn resolve_simple_false() {
        let assumptions = vec!(
            term("p".to_string()),
            implies(
                and(term("p".to_string()), term("q".to_string())),
                term("r".to_string())
            ),
            implies(
                or(term("s".to_string()), term("t".to_string())),
                term("q".to_string())
            )
        );

        let goal = term("r".to_string());

        assert_eq!(resolve(assumptions, goal), false);
    }
}
