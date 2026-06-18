fn network_delay_time_bellman_ford(times: &[(i32, i32, i32)], n: i32, k: i32) -> i32 {
    dist[k] = 0;

    for _ in 0..n - 1 {
        let mut updated = false;
        for &(from, to, weight) in times {
            if dist[from] != i32::MAX && dist[from] + weight < dist[to] {
                dist[to] = dist[from] + weight;
                updated = true;
            }
        }
    }
}