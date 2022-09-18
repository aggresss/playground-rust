type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

pub fn dijkstra<V, E>(graph: &Graph<V, E>, start: &V) -> BTreeMap<V, Option<(V, E)>>
where
    V: Ord + Copy,
    E: Ord + Copy + Add<Output = E>
{
    let mut ans = BTreeMap::new();
    let mut prio = BinaryHeap::new();

    ans.insert(*start, None);

    for (new, weight) in &graph[start] {
        ans.insert(*new, Some((*start, *weight)));
        prio.push(Reverse((*weight, new, start)));
    }

    while let Some(Reverse((dist_new, new, prev))) = prio.pop() {
        match ans[new] {
            Some((p, d)) if p == *prev && d == dist_new => {}
            _ => continue,
        }

        for (next, weight) in &graph[new] {
            match ans.get(next) {
                Some(Some((_, dist_new))) if dist_new + *weight >= *dist_new => {}
                Some(None) => {}

            }
        }

    }


}