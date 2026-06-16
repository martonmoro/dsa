// Given an array of intervals where intervals[i] = [starti, endi], 
// merge all overlapping intervals, and return an array of the non-overlapping 
// intervals that cover all the intervals in the input.

fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    let n = intervals.len();
    let mut merged = Vec::new();

    for interval in intervals {
        if merged.is_empty() {
            merged.push(interval);
            continue;
        }

        let last = merged.last_mut().unwrap();

        if interval[0] <= last[1] {
            last[1] = last[1].max(interval[1]);
        } else {
            merged.push(interval)
        }
    }

    merged
}

fn main() {
    let intervals = vec![
    vec![1, 3],
    vec![15, 18],
    vec![2, 6],
    vec![8, 10],
    ];

    println!("Merged intervals: {:?}", merge(intervals));
}
