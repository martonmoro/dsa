// There is a new alien language that uses the English alphabet,
// but the order of the letters is unknown.

// You are given a list of strings words from the alien language's
// dictionary. It is claimed that the strings in words are sorted
// lexicographically by the rules of this new language.

// If this claim is incorrect, and the given arrangement of strings
// in words cannot correspond to any order of letters, return "".

// Otherwise, return a string of the unique letters in the new alien
// language sorted in lexicographically increasing order by the new
// language's rules. If there are multiple solutions, return any of them.

// A string a is lexicographically smaller than a string b if either
// of the following is true:

// The first letter where they differ is smaller in a than in b.
// a is a prefix of b and a.length < b.length.

use std::collections::{HashMap, HashSet, VecDeque};

fn foreign_dictionary(words: Vec<String>) -> String {
    let mut graph: HashMap<char, HashSet<char>> = HashMap::new();
    let mut indegree: HashMap<char, i32> = HashMap::new();

    // Initialize all characters
    for word in &words {
        for ch in word.chars() {
            graph.entry(ch).or_default();
            indegree.entry(ch).or_insert(0);
        }
    }

    let mut queue = VecDeque::new();
    let mut result = String::new();

    // Fill graph with "dependencies"
    for i in 0..words.len() - 1 {
        let word1 = &words[i];
        let word2 = &words[i + 1];

        let mut found_diff = false;

        for (c1, c2) in word1.chars().zip(word2.chars()) {
            if c1 != c2 {
                // set.insert returns a bool so we only count them once
                if graph.entry(c1).or_default().insert(c2) {
                    *indegree.entry(c2).or_insert(0) += 1;
                }
                found_diff = true;
                break;
            }
        }

        if !found_diff && word1.len() > word2.len() {
            return "".to_string();
        }
    }

    // Fill queu with possible starter characters
    for (&ch, &indeg) in &indegree {
        if indeg == 0 {
            queue.push_back(ch);
        }
    }

    while let Some(ch) = queue.pop_front() {
        result.push(ch);

        if let Some(neighbors) = graph.get(&ch) {
            for &next in neighbors {
                let deg = indegree.get_mut(&next).unwrap();
                *deg -= 1;

                if *deg == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    if result.len() != indegree.len() {
        return "".to_string();
    }

    result
}

fn main() {
    let words = vec!["hrn", "hrf", "er", "enn", "rfnn"]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    println!("Word: {}", foreign_dictionary(words));
}
