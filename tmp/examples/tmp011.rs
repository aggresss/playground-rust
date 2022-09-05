#![allow(dead_code)]

use std::{collections::{BTreeMap, BinaryHeap}, ops::Add, cmp::Reverse};

fn main() {
    let mut graph = BTreeMap::new();
    add_edge(&mut graph, 'a', 'b', 6);
    add_edge(&mut graph, 'a', 'c', 7);
    add_edge(&mut graph, 'a', 'e', 2);
    add_edge(&mut graph, 'a', 'f', 3);
    add_edge(&mut graph, 'b', 'c', 5);
    add_edge(&mut graph, 'c', 'e', 5);
    add_edge(&mut graph, 'd', 'e', 4);
    add_edge(&mut graph, 'd', 'f', 1);
    add_edge(&mut graph, 'e', 'f', 2);

    let mut ans = BTreeMap::new();
    add_edge(&mut ans, 'd', 'f', 1);
    add_edge(&mut ans, 'e', 'f', 2);
    add_edge(&mut ans, 'a', 'e', 2);
    add_edge(&mut ans, 'b', 'c', 5);
    add_edge(&mut ans, 'c', 'e', 5);

    assert_eq!(prim(&graph), ans);
}

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

pub fn add_edge<V, E>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E)
where
    V: Ord + Copy,
    E: Ord + Copy + Add,
{
    graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, c);
    graph.entry(v2).or_insert_with(BTreeMap::new).insert(v1, c);
}

pub fn prim<V, E>(graph: &Graph<V, E>) -> Graph<V, E>
where
    V: Ord + Copy,
    E: Ord + Copy + Add,
{
    match graph.keys().next() {
        Some(v) => prim_with_start(graph, *v),
        None => Graph::new(),
    }
}

pub fn prim_with_start<V, E>(graph: &Graph<V, E>, start: V) -> Graph<V, E>
where
    V: Ord + Copy,
    E: Ord + Copy + Add,
{
    let mut mst: Graph<V, E> = Graph::new();

    let mut prio = BinaryHeap::new();

    mst.insert(start, BTreeMap::new());

    for (v, c) in &graph[&start] {
        prio.push(Reverse((*c, v, &start)));
    }

    while let Some(Reverse((dist, t, prev))) = prio.pop() {
        if mst.contains_key(t) {
            continue;
        }

        add_edge(&mut mst, *prev, *t, dist);

        for (v, c) in &graph[t] {
            if !mst.contains_key(v) {
                prio.push(Reverse((*c, v, t)));
            }
        }
    }

    mst
}