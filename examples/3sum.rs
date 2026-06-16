// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

// Notice that the solution set must not contain duplicate triplets.
use std::cmp::Ordering;
fn three_sum(mut input: Vec<i32>) -> Vec<Vec<i32>> {
    if input.len() < 3 {
        return vec![];
    }
    let mut result: Vec<Vec<i32>> = Vec::new();
    // Sort array
    input.sort();

    // For each i we will do a 2 pointers seach for the other two elements (if they exist)
    for i in 0..input.len() - 2 {
        if input[i] > 0 {
            break;
        }

        if i > 0 && input[i] == input[i-1] {
            continue;
        }

        let i_val = input[i];
        let mut left = i+1 as usize;
        let mut right = input.len() - 1;

        while left < right{
            let left_val = input[left];
            let right_val = input[right];
            let sum = i_val + left_val + right_val;

            match sum.cmp(&0) {
                Ordering::Greater => {
                    right -= 1;
                },
                Ordering::Less => {
                    left += 1;
                },
                Ordering::Equal => {
                    result.push(vec![i_val, left_val, right_val]);

                    left += 1;
                    right -= 1;

                    // Skip duplicates on the left
                    while left < right && input[left] == input [left-1] {
                        left += 1;
                    }

                    while left < right && input[right] == input [right+1] {
                        right -= 1;
                    }
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