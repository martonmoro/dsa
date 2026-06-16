// There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. 
// You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must 
// take course bi first if you want to take course ai.

// For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
// Return the ordering of courses you should take to finish all courses. If there are many valid 
// answers, return any of them. If it is impossible to finish all courses, return an empty array.

use std::collections::VecDeque;
fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    // Build graph: prereq -> course
    let mut indegree = vec![0; num_courses as usize];
    let mut graph = vec![vec![]; num_courses as usize];

    for prerequisite in prerequisites {
        let course = prerequisite[0] as usize;
        let prereq = prerequisite[1] as usize;

        graph[prereq].push(course);
        indegree[course] += 1;
    }

    // Build a VecDeque with coureses that have indegree 0
    let mut queue = VecDeque::new();
    for i in 0..num_courses {
        if indegree[i as usize] == 0 {
            queue.push_back(i);
        }
    }

    // Go through the VecDeque
    let mut taken = 0;
    let mut result: Vec<i32> = Vec::new();
    while let Some(course) = queue.pop_front() {
        taken += 1;
        result.push(course);
        for &next in graph[course as usize].iter() {
            indegree[next] -= 1;

            if indegree[next] == 0 {
                queue.push_back(next as i32);
            }
        }
    }

    if taken == num_courses {
        result
    } else {
        vec![]
    }

}

fn main() {
    let num_courses = 4;
    let prerequisites = vec![
        vec![1, 0],
        vec![2, 0],
        vec![3, 1],
        vec![3, 2],
    ];

    println!("Order: {:?}", find_order(num_courses, prerequisites));
}