// Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas. 
// The guards have gone and will come back in h hours.

// Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile 
// of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats 
// all of them instead and will not eat any more bananas during this hour.

// Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.

// Return the minimum integer k such that she can eat all the bananas within h hours.
fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    // binary search on the solution space
    let max_pile = *piles.iter().max().unwrap();

    let mut left = 1;
    let mut right = max_pile;

    while left < right {
        let mid = left + (right - left) / 2;
        let mut hours = 0;
        for &pile in &piles {
            hours += (pile + mid - 1) / mid;
        }
        if hours <= h {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left 
}

fn main() {
    let test_cases = vec![
        (vec![3, 6, 7, 11], 8, 4),
        (vec![30, 11, 23, 4, 20], 5, 30),
        (vec![30, 11, 23, 4, 20], 6, 23),
        (vec![1, 1, 1, 1], 4, 1),
        (vec![1000000000], 2, 500000000),
        (vec![312884470], 968709470, 1),
    ];

    for (piles, h, expected) in test_cases {
        let result = min_eating_speed(piles.clone(), h);
        println!(
            "piles={:?}, h={} -> result={}, expected={}",
            piles, h, result, expected
        );
    }
}