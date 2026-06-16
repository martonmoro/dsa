// Given an array of integers temperatures represents the daily temperatures, return an 
// array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. 
// If there is no future day for which this is possible, keep answer[i] == 0 instead.

fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    // Brute-force - O(n^2)
    // let n = temperatures.len();
    // let mut result = vec![0; n];
    // for i in 0..n {
    //     let days = 0;
    //     for j in i+1..n {
    //         if temperatures[j] > temperatures[i] {
    //             result[i] = (j - i) as i32;
    //             break;
    //         }
    //     }
    // }

    // result


    // Monotonic-stack - O(n)
    let n = temperatures.len();
    let mut result = vec![0; n];
    let mut stack = Vec::new();

    for i in 0..n {
        while let Some(&j) = stack.last() {
            if temperatures[i] > temperatures[j] {
                stack.pop();
                result[j] = (i - j) as i32;
            } else {
                break;
            }
        }
        stack.push(i);
    }
    result
}

fn main() {
    let test_cases = vec![
        vec![73, 74, 75, 71, 69, 72, 76, 73], // [1,1,4,2,1,1,0,0]
        vec![30, 40, 50, 60],                 // [1,1,1,0]
        vec![30, 60, 90],                     // [1,1,0]
        vec![90, 80, 70, 60],                 // [0,0,0,0]
        vec![1],                              // [0]
        vec![],                               // []
        vec![50, 50, 50, 50],                 // [0,0,0,0]
        vec![70, 71, 70, 71, 70, 71],         // alternating pattern
    ];

    for temps in test_cases {
        let result = daily_temperatures(temps.clone());
        println!("{:?} -> {:?}", temps, result);
    }
}