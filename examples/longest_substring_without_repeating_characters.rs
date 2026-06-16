// Given a string s, find the length of the longest substring without duplicate characters.

use std::collections::HashSet;
fn length_of_longest_substring(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let mut seen: HashSet<char> = HashSet::new();

    let mut left = 0;
    let mut max = 0;

    for right in 0..s.len() {
        while seen.contains(&s[right]) {
            seen.remove(&s[left]);
            left += 1;
        }

        seen.insert(s[right]);

        max = max.max((right - left + 1) as i32);
    }

    max 
}

fn main() {
    let input = "pwwkew".to_string();
    assert_eq!(3, length_of_longest_substring(input));
}