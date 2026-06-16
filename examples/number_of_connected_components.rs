fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    // Build graph
    let mut graph = vec![vec![]; n];

    for edge in &edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;

        graph[a].push(b);
        graph[b].push(a);
    }

    let mut visited = vec![false; n];
    let mut count = 0;

    for node in 0..n {
        if !visited[node] {
            count += 1;
            dfs(node, &graph, &mut visited);
        }
    }

    count
}

fn dfs(node: usize, graph: &[Vec<usize>], visited: &mut [bool]) {
    visited[node] = true;
    for &next in &graph[node] {
        if !visited[next] {
            dfs(next, graph, visited);
        }
    }
}

////////////////////////////////
fn main() {
    let n = 5;

    let edges = vec![
        vec![0, 1],
        vec![1, 2],
        vec![3, 4],
    ];

    let components = count_components(n, edges);

    println!("Number of connected components: {}", components);
}