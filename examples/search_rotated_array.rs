// There is an integer array nums sorted in ascending order (with distinct values).

// Prior to being passed to your function, nums is possibly left rotated at an unknown 
// index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). 
// For example, [0,1,2,4,5,6,7] might be left rotated by 3 indices and become [4,5,6,7,0,1,2].

// Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.

// You must write an algorithm with O(log n) runtime complexity.

fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let n = nums.len();
    let mut left = 0;
    let mut right = n-1;

    while left <= right {
        let mid = left + (right - left) / 2;
        let l = nums[left];
        let r = nums[right];
        let m = nums[mid];

        if m == target {
            return mid as i32;
        }

        if l <= m {
            if target <= m && target >= l {
                right = mid -1;
            } else {
                left = mid + 1;
            }
        } else {
            if target <= r && target >= m {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    -1
}

fn main() {
    let test_cases = vec![
        (vec![4,5,6,7,0,1,2], 0, 4),
        (vec![4,5,6,7,0,1,2], 3, -1),
        (vec![1], 0, -1),
        (vec![1], 1, 0),
        (vec![1,2,3,4,5,6,7], 5, 4),
        (vec![5,1,2,3,4], 1, 1),
        (vec![5,1,2,3,4], 5, 0),
        (vec![6,7,1,2,3,4,5], 3, 4),
        (vec![6,7,1,2,3,4,5], 8, -1),
    ];

    for (nums, target, expected) in test_cases {
        let result = search(nums.clone(), target);
        println!(
            "nums={:?}, target={} -> result={}, expected={}",
            nums, target, result, expected
        );
    }
}