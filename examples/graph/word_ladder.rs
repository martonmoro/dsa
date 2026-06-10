// A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:

// Every adjacent pair of words differs by a single letter.
// Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
// sk == endWord
// Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.

use std::collections::{HashSet, VecDeque};
fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_set : HashSet<String> = word_list.into_iter().collect();

        if !word_set.contains(&end_word) {
            return 0;
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((begin_word.clone(), 1));
        visited.insert(begin_word);

        while let Some((word, steps)) = queue.pop_front() {
            let mut chars: Vec<char> = word.chars().collect();

            for i in 0..chars.len() {
                let original = chars[i];

                for ch in b'a'..=b'z' {
                    let new_char = ch as char;
                    if new_char == original {
                        continue;
                    }

                    chars[i] = new_char;

                    let next: String = chars.iter().collect();

                    if word_set.contains(&next) && !visited.contains(&next) {
                        if next == end_word {
                            return steps + 1;
                        }

                        visited.insert(next.clone());
                        queue.push_back((next, steps + 1));
                    }
                }

                chars[i] = original;
            }
        }

        0
    }

fn main() {
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list: Vec<String> = vec!["hot", "dot", "lot", "log", "cog"].into_iter().map(|x| x.to_string()).collect();

    assert_eq!(5, ladder_length(begin_word, end_word, word_list));
}