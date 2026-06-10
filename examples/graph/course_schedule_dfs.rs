// There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.

// For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
// Return true if you can finish all courses. Otherwise, return false.

fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {

    let n = num_courses as usize;

    let mut graph = vec![vec![]; n];

    for prereq in prerequisites {
        let course = prereq[0] as usize;
        let prereq = prereq[1] as usize;
    
        // prereq -> course
        graph[prereq]. push(course);
    }
    
    // 0 - not visited
    // 1 - visiting 
    // 2 - visited
    let mut state = vec![0; n];
    for course in 0..n {
        if !dfs(course, &graph, &mut state) {
            return false;
        }
    }

    true
}

fn dfs(node: usize, graph: &Vec<Vec<usize>>, state: &mut Vec<i32>) -> bool {
    match state[node] {
        0 => {
            state[node] = 1; // currently visiting

            for &next in &graph[node] {
                if !dfs(next, &graph, state) {
                    return false;
                }
            }

            state[node] = 2;
            true
        },
        1 => {
            false
        },
        2 => {
            true
        },
        _ => unreachable!()
    }
}

fn main() {
    assert_eq!(true, can_finish(2, vec![vec![1, 0]]));
    assert_eq!(false, can_finish(2, vec![vec![1, 0], vec![0,1]]));
}