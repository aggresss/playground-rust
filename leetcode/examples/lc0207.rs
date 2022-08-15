#![allow(dead_code)]

fn main() {}

struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num = num_courses as usize;
        let mut matrix = vec![vec![false; num]; num];
        let mut in_degree = vec![0; num];

        for pre in prerequisites.iter() {
            matrix[pre[1] as usize][pre[0] as usize] = true;
            in_degree[pre[0] as usize] += 1;
        }

        let mut deq = VecDeque::new();
        for (node, &v) in in_degree.iter().enumerate() {
            if v == 0 {
                deq.push_back(node);
            }
        }

        let mut cnt = 0;
        while let Some(node) = deq.pop_front() {
            cnt += 1;
            for (index, &connect) in matrix[node].iter().enumerate() {
                if connect {
                    in_degree[index] -= 1;
                    if in_degree[index] == 0 {
                        deq.push_back(index);
                    }
                }
            }
        }

        cnt == num_courses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
        assert_eq!(
            Solution::can_finish(
                8,
                vec![
                    vec![1, 0],
                    vec![2, 6],
                    vec![1, 7],
                    vec![6, 4],
                    vec![7, 0],
                    vec![0, 5]
                ]
            ),
            true
        );
    }
}
