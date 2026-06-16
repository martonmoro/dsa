// There are n cities connected by some number of flights. 
// You are given an array flights where flights[i] = [fromi, toi, pricei] 
// indicates that there is a flight from city fromi to city toi with cost pricei.

// You are also given three integers src, dst, and k, return the cheapest 
// price from src to dst with at most k stops. If there is no such route, return -1.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn find_cheapest_price(
    n: i32,
    flights: Vec<Vec<i32>>,
    src: i32,
    dst: i32,
    k: i32,
) -> i32 {
    let n = n as usize;
    let src = src as usize;
    let dst = dst as usize;

    // graph
    let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];

    for flight in flights {
        let from = flight[0] as usize;
        let to = flight[1] as usize;
        let price = flight[2];

        graph[from].push((to, price));
    }

    let mut best = vec![vec![i32::MAX; (k + 2) as usize]; n];
    best[src][0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, src, 0))); // (cost, city, flights_used)

    while let Some(Reverse((cost, city, flights_used))) = heap.pop() {
        if city == dst {
            return cost;
        }

        if flights_used == (k + 1) as usize {
            continue;
        }

        for &(next, price) in &graph[city] {
            let next_cost = cost + price;
            let next_flights = flights_used + 1;

            if next_cost < best[next][next_flights] {
                best[next][next_flights] = next_cost;
                heap.push(Reverse((next_cost, next, next_flights)));
            }
        }
    }

    -1
}

fn main() {
    let n = 4;

    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 3, 100],
        vec![0, 3, 500],
    ];

    let src = 0;
    let dst = 3;
    let k = 1;

    let result = find_cheapest_price(n, flights, src, dst, k);

    println!("Cheapest price: {}", result);
}