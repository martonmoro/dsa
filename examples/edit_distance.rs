// Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.

// You have the following three operations permitted on a word:

// Insert a character
// Delete a character
// Replace a character

fn edit_distance(word1: String, word2: String) -> i32 {
    let a: Vec<char> = word1.chars().collect();
    let b: Vec<char> = word2.chars().collect();
    let m = a.len();
    let n = b.len();

    // dp[i][j] = min edits to turn the first i chars of a into the first j chars of b
    let mut dp = vec![vec![0i32; n+ 1]; m + 1];

    // Base cases
    for i in 0..=m {
        dp[i][0] = i as i32;
    }
    for j in 0..=n {
        dp[0][j] = j as i32;
    }

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                // Last chars match: no new cost, inherit diagonal
                dp[i][j] = dp[i-1][j-1];
            } else {
                let delete = dp[i-1][j];
                let insert = dp[i][j -1];
                let replace = dp[i-1][j-1];
                dp[i][j] = 1 + delete.min(insert).min(replace);
            }
        }
    }

    dp[m][n]
}

/////////////////////////////////////////////////

fn main() {
    let tests = vec![
        ("horse", "ros", 3),
        ("intention", "execution", 5),
        ("", "abc", 3),
        ("abc", "", 3),
        ("abc", "abc", 0),
        ("a", "b", 1),
    ];
 
    for (w1, w2, expected) in tests {
        let result = edit_distance(w1.to_string(), w2.to_string());
        println!(
            "min_distance({:?}, {:?}) = {} (expected {})",
            w1, w2, result, expected
        );
        assert_eq!(result, expected);
    }
 
    println!("All test cases passed ✅");
}