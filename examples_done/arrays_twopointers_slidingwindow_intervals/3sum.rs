// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

// Notice that the solution set must not contain duplicate triplets.

use std::cmp::Ordering;

fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    nums.sort();

    let n = nums.len();

    if n < 3 {
        return vec![];
    }

    // Pick first element
    for i in 0..n {
        // if first is positive we can stop early
        if nums[i] > 0 {
            break;
        }

        // skip duplicate first element
        if i > 0 && nums[i] == nums[i-1] {
            continue;
        }

        // two-pointer search on the remaining portion of the array
        let mut left = i + 1;
        let mut right = n - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            match sum.cmp(&0) {
                Ordering::Equal => {
                    // Found a valid triple
                    result.push(vec![nums[i], nums[left], nums[right]]);

                    // Move both pointers inward
                    left += 1;
                    right -= 1;

                    // Skip duplicate on left
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }

                    // Skip duplicate on right
                    while right < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                },
                Ordering::Less => {
                    // Sum is too small
                    left += 1;
                },
                Ordering::Greater => {
                    // Sum is too large
                    right -= 1;
                }

            }
        }
    }

    result
}

fn main() {
    let result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
    println!("Result: {:?}", result);
}