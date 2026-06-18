// Given an integer array nums, find the subarray with the largest sum, and return its sum.

fn max_subarray(nums: &[i32]) -> i32 {
    // 3 cases
    // 1. max subarray in left half
    // 2. max subarray in right half
    // 3. max subarray is (max suffix subarray of left half) + (max prefix subarray of right half)
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }

    // max left 
    let left = max_subarray(&nums[0..n/2]);

    // max right
    let right = max_subarray(&nums[n/2..]);

    let mid = {
        // walk left from mid
        let mut suffix = 0;
        let mut max_suffix = i32::MIN;
        for i in (0..n/2).rev() {
            suffix += nums[i];
            max_suffix = max_suffix.max(suffix);
        }
        // walk right from mid+1
        let mut prefix = 0;
        let mut max_prefix = i32::MIN;
        for i in n/2..n {
            prefix += nums[i];
            max_prefix = max_prefix.max(prefix);
        }
        max_suffix + max_prefix
    };

    left.max(right).max(mid)
}

fn max_subarray_kadane(nums: &[i32]) -> i32 {
    let mut current = nums[0];
    let mut best = nums[0];

    for &x in &nums[1..] {
        current = x.max(current + x);
        best = best.max(current);
    }

    best
}

fn main() {
    let test_cases = vec![
        vec![1],                          // single element
        vec![-1],                         // single negative
        vec![1, 2],                       // two elements
        vec![-1, -2],                     // two negatives

        vec![1, -2, 3],                   // answer in right half
        vec![3, -2, 1],                   // answer in left half

        vec![2, -1, 2],                   // crossing midpoint
        vec![2, -1, 3, -2, 4],            // larger crossing example

        vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], // classic example -> 6

        vec![-8, -3, -6, -2, -5, -4],    // all negative

        vec![1, 2, 3, 4, 5],             // whole array

        vec![8, -19, 5, -4, 20],         // crossing beats both halves

        vec![100, -1, -1, -1, -1],       // left half wins

        vec![-1, -1, -1, -1, 100],       // right half wins
    ];

    for nums in test_cases {
        println!("{:?} -> {}", nums, max_subarray(&nums));
    }
}