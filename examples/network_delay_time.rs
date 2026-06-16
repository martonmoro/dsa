// You are given a network of n nodes, labeled from 1 to n. 
// You are also given times, a list of travel times as directed 
// edges times[i] = (ui, vi, wi), where ui is the source node, 
// vi is the target node, and wi is the time it takes for a signal 
// to travel from source to target.

// We will send a signal from a given node k. Return the minimum 
// time it takes for all the n nodes to receive the signal. If it 
// is impossible for all the n nodes to receive the signal, return -1.

use std::collections::BinaryHeap;
use std::cmp::Reverse;
fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    // Build graph 
    let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; (n+1) as usize]; // n+1 because nodes are 1-indexed
    for time in times {
        let from = time[0] as usize;
        let to = time[1] as usize;
        let weight = time[2];

        // from -> (to, weight)
        graph[from].push((to, weight));
    }

    // shortest known time from k to i
    let mut dist = vec![i32::MAX; (n + 1) as usize];
    dist[k as usize] = 0;

    // Min-heap (priority queue)
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    // Start from source node k with cost 0
    heap.push(Reverse((0, k)));

    // Dijkstra
    while let Some(Reverse((cost, node))) = heap.pop() {
        // If we already found a better path to this node, skip this
        if cost > dist[node] {
            continue;
        }

        // Explore all neighbours of current node
        for &(next, weight) in &graph[node] {
            // Compute new possible cost to reach next
            let next_cost = cost + weight;

            // If we found a shorter path, update it
            if next_cost < dist[next] {
                dist[next] = next_cost;

                // Push updated distance
                heap.push(Reverse((next_cost, next)));
            }
        }
    }

    // We want time it takes for the last node to receive signal, so maximum shortest path
    let mut answer = 0;
    for &d in dist.iter().skip(1) {
        if d == i32::MAX {
            return -1;
        }

        answer = answer.max(d);
    }

    answer
}

fn main() {
    let times = vec![
        vec![2, 1, 1],
        vec![2, 3, 1],
        vec![3, 4, 1],
    ];

    let n = 4;
    let k = 2;

    let result = network_delay_time(times, n, k);

    println!("Network delay time: {}", result);
}