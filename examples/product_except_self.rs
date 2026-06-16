// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

// You must write an algorithm that runs in O(n) time and without using the division operation.

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // Fill up the prefix array
    let mut result = vec![0; nums.len()];
    let mut prefix = 1;
    for i in 0..nums.len() {
        result[i] = prefix;
        prefix *= nums[i];
    }

    // Multiply by suffixes
    let mut suffix = 1;
    for j in (0..nums.len()).rev() {
        result[j] = result[j] * suffix;
        suffix *= nums[j];
    }

    result
}

fn main() {
    let input_1 = vec![1,2,3,4];
    let input_2 = vec![-1,1,0,-3,3];

    println!("Result 1: {:?}", product_except_self(input_1));
    println!("Result 2: {:?}", product_except_self(input_2));
}