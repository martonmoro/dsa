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

    let mut dist = vec![i32::MAX; n];
    dist[src] = 0;

    // allow k + 1 edges
    for _ in 0..=k {
        let mut temp = dist.clone();

        for flight in &flights {
            let from = flight[0] as usize;
            let to = flight[1] as usize;
            let cost = flight[2];

            if dist[from] != i32::MAX {
                temp[to] = temp[to].min(dist[from] + cost);
            }
        }

        dist = temp;
    }

    if dist[dst] == i32::MAX {
        -1
    } else {
        dist[dst]
    }
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