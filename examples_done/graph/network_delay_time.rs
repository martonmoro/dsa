// You are given a network of n nodes, labeled from 1 to n. 
// You are also given times, a list of travel times as directed 
// edges times[i] = (ui, vi, wi), where ui is the source node, 
// vi is the target node, and wi is the time it takes for a signal 
// to travel from source to target.

// We will send a signal from a given node k. Return the minimum 
// time it takes for all the n nodes to receive the signal. If it 
// is impossible for all the n nodes to receive the signal, return -1.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    // Build graph (nodes are numbered 1..n)
    let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; (n + 1) as usize];
    for time in times {
        let from = time[0] as usize;
        let to = time[1] as usize;
        let weight = time[2];

        graph[from].push((to, weight));
    }

    let mut dist = vec![i32::MAX; (n + 1) as usize];
    dist[k as usize] = 0;

    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, k as usize)));


    // Dijkstra
    while let Some(Reverse((cost, node))) = heap.pop() {
        if cost > dist[node] {
            continue;
        }

        for &(next, weight) in &graph[node] {
            let next_cost = cost + weight;
            if next_cost < dist[next] {
                dist[next] = next_cost;
                heap.push(Reverse((next_cost, next)));
            }
        }
    }

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