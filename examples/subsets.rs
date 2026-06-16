// Given an integer array nums of unique elements, return all possible subsets (the power set).

// The solution set must not contain duplicate subsets. Return the solution in any order.
fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    backtrack(0, &nums, &mut current, &mut result);
    result
}

fn backtrack(
    start: usize,
    nums: &[i32],
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    // Every node in the tree is itself a valid subset, so record on entry
    result.push(current.clone());

    // Choices: each element from `start` onward.
    for i in start..nums.len() {
        current.push(nums[i]); // make the choice
        backtrack(i + 1, nums, current, result); // recourse with start = i + 1
        current.pop(); // undo the choice
    }
}

/////////////////////////////////////////

fn main() {
    let nums = vec![1, 2, 3];

    let result = subsets(nums);

    println!("Subsets:");
    for subset in result {
        println!("{:?}", subset);
    }
}