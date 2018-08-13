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

    println!("neg_goal_clauses: {:?}", neg_goal_clauses);
    for c in neg_goal_clauses {
        println!("resolve: {:?}", c);

        let mut visited = HashSet::new();
        visited.insert(&c);

        if resolve_(&clauses, &c, visited) {
            return true
        }
    }
    false
}

fn resolve_(clauses: &ClauseStorage, current: &Clause, visited: HashSet<&Clause>) -> bool {
    println!("resolve_: {:?}", current);
    println!("resolve_ clauses: {:?}", clauses);
    for p in current.parts.clone() {
        println!("resolve_ p: {:?}", p);
        for m in clauses.get(&p.negate(), &visited) {
            println!("resolve_ m: {:?}", m);
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
    println!("combine: {:?}", all_parts);

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

    fn get(&self, part: &ClausePart, visited: &HashSet<&Clause>) -> Option<&Clause> {
        println!("get: {:?}\n{:?}\n{:?}", part, visited, self.lookup_table);
        match self.lookup_table.get_vec(part) {
            Some(indices) => {
                for i in indices {
                    let value = &self.clauses[*i];

                    if !visited.contains(&value) {
                        return Some(&value)
                    }
                }
                None
            }
            None => None
        }
    }

    fn put(&mut self, clause: Clause) {
        let index = self.clauses.len();

        clause.parts.iter()
            .for_each(|p| self.lookup_table.insert((*p).clone(), index));

        self.clauses.push(clause);
    }
}

fn multimap_pop<'a, K, V>(multimap: &'a mut MultiMap<K, V>, key: &K) -> Option<V>
    where K: Eq + Hash + Clone
{
    match multimap.remove(key) {
        Some(mut values) => {
            let ret = values.pop().unwrap();

            values.into_iter()
                .for_each(|v| multimap.insert(key.clone(), v));

            Some(ret)
        },
        None => None
    }
}

#[derive(Debug)]
struct ClauseEntry {
    clause: Clause,
    valid: bool
}

impl ClauseEntry {
    fn new(clause: Clause) -> ClauseEntry {
        ClauseEntry {
            clause,
            valid: true
        }
    }

    fn get(&self) -> Option<&Clause> {
        if self.valid {
            Some(&self.clause)
        } else {
            None
        }
    }

    fn expire(&mut self) {
        self.valid = false;
    }
}

#[cfg(test)]
mod tests {
    use clauses::*;
    use propositions::*;
    use resolution::*;

    #[test]
    fn resolve_simple_true() {
        let assumptions = vec!(
            term("a".to_string())
        );

        let goal = term("a".to_string());

        assert_eq!(resolve(assumptions, goal), true);
    }
}
