// You are given an array of variable pairs equations and an array of
// real numbers values, where equations[i] = [Ai, Bi] and values[i]
// represent the equation Ai / Bi = values[i]. Each Ai or Bi is a string that represents a single variable.

// You are also given some queries, where queries[j] = [Cj, Dj]
// represents the jth query where you must find the answer for Cj / Dj = ?.

// Return the answers to all queries. If a single answer cannot be determined, return -1.0.

// Note: The input is always valid. You may assume that evaluating the
// queries will not result in division by zero and that there is no contradiction.

// Note: The variables that do not occur in the list of equations are undefined,
// so the answer cannot be determined for them.

use std::collections::{HashMap, HashSet};

fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    // We have to build a weighted graph where the variables are the nodes and the eqns are the weighs
    // Variables are nodes.
    // Equation a / b = k becomes:
    // a -> b (k)
    // b -> a (1/k)
    for (eq, val) in equations.iter().zip(values.iter()) {
        let a = eq[0].clone();
        let b = eq[1].clone();

        graph.entry(a.clone()).or_default().push((b.clone(), *val));
        graph
            .entry(b.clone())
            .or_default()
            .push((a.clone(), 1.0 / *val));
    }

    queries
        .into_iter()
        .map(|query| {
            let start = &query[0];
            let end = &query[1];

            if !graph.contains_key(start) || !graph.contains_key(end) {
                return -1.0;
            }

            if start == end {
                return 1.0;
            }

            let mut visited = HashSet::new();

            dfs(start, end, 1.0, &mut visited, &graph).unwrap_or(-1.0)
        })
        .collect()
}

fn dfs(
    current: &str,
    target: &str,
    product: f64,
    visited: &mut HashSet<String>,
    graph: &HashMap<String, Vec<(String, f64)>>,
) -> Option<f64> {
    if current == target {
        return Some(product);
    }

    visited.insert(current.to_string());

    if let Some(neighbors) = graph.get(current) {
        for (next, weight) in neighbors {
            if visited.contains(next) {
                continue;
            }

            if let Some(result) = dfs(next, target, product * *weight, visited, graph) {
                return Some(result);
            }
        }
    }

    None
}

fn main() {
    let equations = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
    ];

    let values = vec![2.0, 3.0];

    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["c".to_string(), "a".to_string()],
        vec!["a".to_string(), "e".to_string()],
        vec!["a".to_string(), "a".to_string()],
        vec!["x".to_string(), "x".to_string()],
    ];

    let results = calc_equation(equations, values, queries);

    println!("{results:?}");
}
