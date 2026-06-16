// You are given an integer array coins representing coins of different 
// denominations and an integer amount representing a total amount of money.

// Return the fewest number of coins that you need to make up that amount. 
// If that amount of money cannot be made up by any combination of the coins, return -1.

// You may assume that you have an infinite number of each kind of coin.

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;

    // dp[a] = fewest coins to make amount `a`.
    // Initialize with a sentinel "infinity" = amount + 1, which is larger
    // than any real answer (you'd never need more than `amount` coins).
    let mut dp = vec![amount + 1; amount + 1];
    dp[0] = 0; // base case: zero coins make amount 0

    for a in 1..=amount {
        for &coin in &coins {
            let coin = coin as usize;
            if coin <= a {
                // "use this coin": 1 + best way to make the remainder
                dp[a] = dp[a].min(dp[a - coin] + 1);
            }
        }
    }

    // If dp[amount] is still the sentinel, no combination worked.
    if dp[amount] > amount {
        -1
    } else {
        dp[amount] as i32
    }
}

////////////////////////////////////////////

fn main() {
    let tests = vec![
        (vec![1, 2, 5], 11),
        (vec![2], 3),
        (vec![1], 0),
        (vec![1], 1),
        (vec![1], 2),
    ];

    for (coins, amount) in tests {
        let result = coin_change(coins.clone(), amount);

        println!(
            "coins = {:?}, amount = {} => {}",
            coins, amount, result
        );
    }
}