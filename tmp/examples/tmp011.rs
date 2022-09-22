use std::{collections::{BTreeMap, BinaryHeap}, ops::Add, cmp::Reverse};

fn main() {}

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

pub fn dijkstra<V, E>(graph: &Graph<V, E>, start: &V) -> BTreeMap<V, Option<(V, E)>>
where
    V: Ord + Copy,
    E: Ord + Copy + Add<Output = E>,
{
    let mut prio = BinaryHeap::new();
    let mut ans = BTreeMap::new();

    ans.insert(*start, None);
    for (new, weight) in &graph[start] {
        ans.insert(*new, Some((*start, *weight)));
        prio.push(Reverse((*weight, new, start)));
    }

    while let Some(Reverse((dist_new, new, _))) = prio.pop() {
        for (next, weight) in &graph[new] {
            match ans.get(next) {
                Some(Some((_, next_dist))) if dist_new + *weight >= *next_dist => {}
                Some(None) => {}
                _ => {
                    prio.push(Reverse((dist_new + *weight, next, new)));
                    ans.insert(*next, Some((*new, dist_new + *weight)));
                }
            }
        }
    }

    ans
}

