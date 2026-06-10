// Given a string s, find the length of the longest substring without duplicate characters.

use std::collections::HashSet;

fn length_of_longest_substring(s: String) -> usize {
    let mut set = HashSet::new();

    let chars: Vec<char> = s.chars().collect();

    let mut left: usize = 0;
    let mut max_len = 0;

    for right in 0..chars.len() {
        while set.contains(&chars[right]) {
            set.remove(&chars[left]);
            left += 1;
        }

        set.insert(chars[right]);
        max_len = max_len.max(right - left + 1);
    }

    max_len
}

use std::collections::HashMap;
#[allow(dead_code)]
fn length_of_longest_substring_hashmap(s: String) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut map = HashMap::new();

    let mut left = 0;
    let mut max_len = 0;

    for right in 0..chars.len() {
        if let Some(&prev) = map.get(&chars[right]) {
            left = left.max(prev + 1);
        }

        map.insert(chars[right], right);
        max_len = max_len.max(right - left + 1);
    }

    max_len
}

fn main() {
    let input = "pwwkew".to_string();
    assert_eq!(3, length_of_longest_substring(input));
}