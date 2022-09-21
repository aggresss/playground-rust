use std::{collections::{BTreeMap, BinaryHeap}, cmp::Reverse, ops::Add};


fn main() {}

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

pub fn dijkstra<V, E>(graph: &Graph<V, E>, start: &V) -> BTreeMap<V, Option<V, E>>
where
    V: Ord + Copy,
    E: Ord + Copy + Add<Output = E>
{
    let mut ans = BTreeMap::new();
    let mut prio = BinaryHeap::new();

    ans.insert(*start, None);

    for (new, new_weight) in &graph[start] {
        ans.insert(*new, Some((*start, *new_weight)));
        prio.push(Reverse((*new_weight, new, start)));
    }

    while let Some(Reverse((dist_new, new, prev))) = prio.pop() {
        for (next, next_weight) in &graph[new] {
            match ans.get(next) {
                Some(Some((_, dist_next))) if dist_new + next_weight >= dist_next => {}
                Some(None) => {}
                _ => {
                    ans.insert(*next, Some((*new, dist_new + *next_weight)));
                    prio.push(Reverse((dist_new + *next_weight, next, new)));
                }
            }
        }
    }

    ans
}
