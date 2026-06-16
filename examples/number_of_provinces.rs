// There are n cities. Some are connected, some are not. If city a is connected
// directly with city b, and city b is connected directly with city c, then a and c
// are connected indirectly.
 
// A province is a group of directly or indirectly connected cities and no other
// cities outside of the group.
 
// You are given an n x n matrix is_connected where is_connected[i][j] = 1 if the ith
// city and the jth city are directly connected, and 0 otherwise.
 
// Return the total number of provinces.

fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();

    let mut parent: Vec<usize> = (0..n).collect();
    let mut count = n as i32;

    // FIND with path compression
    fn find(x: usize, parent: &mut Vec<usize>) -> usize {
        if parent[x] != x {
            parent[x] = find(parent[x], parent);
        }
        parent[x]
    }

    // UNION (no rank)
    fn union(x: usize, y: usize, parent: &mut Vec<usize>, count: &mut i32) {
        let px = find(x, parent);
        let py = find(y, parent);

        if px == py {
            return;
        }

        // this is where we could use rank to attack one under the other
        // if rank[px] < rank[py] {
        //     parent[px] = py;
        // } else if rank[px] > rank[py] {
        //     parent[py] = px;
        // } else {
        //     parent[py] = px;
        //     rank[px] += 1;
        // }
        parent[py] = px;
        *count -= 1;
    }

    for i in 0..n {
        for j in (i+1)..n {
            if is_connected[i][j] == 1{
                union(i, j, &mut parent, &mut count);
            }
        }
    }

    count
}

fn main() {
    // More interesting graph:
    //
    // 0 - 1 - 2 - 3
    //         |
    //         4
    //
    // 5 - 6
    //
    // Expected provinces = 2
    // {0,1,2,3,4} and {5,6}

    let is_connected = vec![
        vec![1, 1, 0, 0, 0, 0, 0], // 0
        vec![1, 1, 1, 0, 0, 0, 0], // 1
        vec![0, 1, 1, 1, 1, 0, 0], // 2
        vec![0, 0, 1, 1, 0, 0, 0], // 3
        vec![0, 0, 1, 0, 1, 0, 0], // 4
        vec![0, 0, 0, 0, 0, 1, 1], // 5
        vec![0, 0, 0, 0, 0, 1, 1], // 6
    ];

    let result = find_circle_num(is_connected);

    println!("Number of provinces: {}", result);
}