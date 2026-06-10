// There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.

// For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
// Return true if you can finish all courses. Otherwise, return false.

use std::collections::VecDeque;

fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut graph = vec![vec![]; num_courses as usize];
    let mut indegree = vec![0; num_courses as usize];

    // prerequisites.len() == 2
    // build graph
    // update indegree
    for prereq in prerequisites {
        graph[prereq[1] as usize].push(prereq[0]);
        indegree[prereq[0] as usize] += 1;
    }

    let mut queue = VecDeque::new();

    for course in 0..num_courses {
        if indegree[course as usize] == 0 {
            queue.push_back(course);
        }
    }

    let mut taken = 0;

    while let Some(course) = queue.pop_front() {
        taken += 1;

        for &next in &graph[course as usize] {
            indegree[next as usize] -= 1;

            if indegree[next as usize] == 0 {
                queue.push_back(next);
            }
        }
    }

    taken == num_courses
}

fn main() {
    assert_eq!(true, can_finish(2, vec![vec![1, 0]]));
    assert_eq!(false, can_finish(2, vec![vec![1, 0], vec![0,1]]));
}