pub struct StronglyconnectedComponents {
    pub component: Vec<usize>,
    pub state: Vec<u64>,
    pub num_components: usize,
    stack: Vec<usize>,
    current_time: usize,
}

const NOT_DONE: u64 = 1 << 63;

#[inline]
fn set_done(vertex_state: &mut u64) {
    *vertex_state ^= NOT_DONE;
}

#[inline]
fn is_in_stack(vertex_state: u64) -> bool {
    vertex_state != 0 && (vertex_state & NOT_DONE) != 0
}

#[inline]
fn is_unvisited(vertex_state: u64) -> u64 {
    vertex_state ^ NOT_DONE
}

