use std::collections::{BTreeMap, BinaryHeap};
use std::ops::Add;
use std::cmp::Reverse;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

pub fn add_edge<V, E>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E)
where
    V: Ord + Copy,
    E: Ord + Add + Copy,
{
    graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, c);
    graph.entry(v2).or_insert_with(BTreeMap::new).insert(v1, c);
}

pub fn prim<V, E>(graph: &Graph<V, E>) -> Graph<V, E>
where
    V: Ord + Copy + std::fmt::Debug,
    E: Ord + Add + Copy + std::fmt::Debug,
{
    match graph.keys().next() {
        Some(v) => prim_with_start(graph, *v),
        None => BTreeMap::new(),
    }
}

pub fn prim_with_start<V, E>(graph: &Graph<V, E>, start: V) -> Graph<V, E>
where
    V: Ord + Copy,
    E: Ord + Add + Copy,
{
    let mut mst: Graph<V, E> = Graph::new();
    let mut prio = BinaryHeap::new();

    mst.insert(start, BTreeMap::new());


    for (v, c) in &graph[&start] {
        prio.push(Reverse((*c, v, start)));
    }

    while let Some(Reverse((dist, t, prev))) = prio.pop() {
        if mst.contains_key(t) {
            continue;
        }

        add_edge(&mut mst, prev, *t, dist);

        for (v, c) in &graph[t] {
            if !mst.contains_key(v) {
                prio.push(Reverse((*c, v, *t)));
            }
        }
    }

    mst
}
