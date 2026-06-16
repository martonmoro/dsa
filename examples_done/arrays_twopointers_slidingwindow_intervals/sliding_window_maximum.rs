// You are given an array of integers nums, there is a sliding window of size k which is 
// moving from the very left of the array to the very right. You can only see the k numbers 
// in the window. Each time the sliding window moves right by one position.

// Return the max sliding window.

use std::collections::VecDeque;

fn max_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();

    let mut dq = VecDeque::new();
    let mut result = Vec::new();

    for i in 0..n {
        while let Some(&idx) = dq.front() {
            if idx + k <= i {
                dq.pop_front();
            } else {
                break;
            }
        }

        while let Some(&idx) = dq.back() {
            if nums[idx] <= nums[i] {
                dq.pop_back();
            } else {
                break;
            }
        }

        dq.push_back(i);

        if i + 1 >= k {
            result.push(nums[*dq.front().unwrap()]);
        }
    }

    result

}

fn main() {
    let nums = vec![1,3,-1,-3,5,3,6,7];
    let k = 3;

    println!("Max sliding window: {:?}", max_window(nums, k));
}