// Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.

// A subarray is a contiguous non-empty sequence of elements within an array.
use std::collections::HashMap;
fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    // Brute-force O(n^2)
    // let mut result = 0;
    // for start in 0..nums.len() {
    //     let mut sum = 0;
    //     for end in start..nums.len() {
    //         sum += nums[end];
    //         if sum == k {
    //             result += 1;
    //         }
    //     }
    // }

    // result

    ////////////////////////////////////////////////////////////////////

    // Prefix(end) - Prefix(start) = k
    let mut count = 0;
    let mut prefix_sum = 0;

    // prefix_sum -> frequency
    let mut map = HashMap::new();
    map.insert(0, 1);

    for num in nums {
        prefix_sum += num;
        if let Some(fr) = map.get(&(prefix_sum - k)) {
            count += fr;
        }
        *map.entry(prefix_sum).or_insert(0) += 1;
    }

    count

}

fn main() {
    let nums = vec![1, 2, -2, -1, 3];
    println!("Result: {}", subarray_sum(nums, 3));
}