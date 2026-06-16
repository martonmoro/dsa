// You are given an array of meeting time intervals where each interval is represented as 
// intervals[i] = [start_i, end_i]. Each interval indicates when a meeting starts and ends.

// Your task is to find the minimum number of conference rooms required to schedule all the 
// meetings without any conflicts. In other words, if two or more meetings overlap in time, they need separate conference rooms.

use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn min_conf_room(mut intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.is_empty() {
        return 0;
    }

    intervals.sort_by_key(|interval| interval[0]);

    // we need a min heap because we are going to track the earliest ending meeting
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for interval in intervals {
        let start = interval[0];
        let end = interval[1];

        // peek - 1, push - logn, pop - logn
        if let Some(&Reverse(earliest_end)) = heap.peek() {
            if start >= earliest_end {
                heap.pop();
            }
        }

        heap.push(Reverse(end));
    }

    heap.len() as i32
}

fn main() {
    let intervals = vec![
    vec![0, 30],
    vec![5, 10],
    vec![15, 20],
    vec![12, 16],
    ];

    println!{"Meeting rooms needed: {}", min_conf_room(intervals)};
}