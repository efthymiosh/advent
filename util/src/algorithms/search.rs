use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    cost: u32,
    pos: (usize, usize),
    d: (isize, isize),
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub fn dijkstra<T, F>(grid: &Vec<Vec<T>>, advance: &F, start: (usize, usize), end: (usize, usize), wall: T) -> Option<u32>
where T: Eq + Copy,
F: Fn(u32, (usize, usize), (isize, isize)) -> Vec<(u32, (usize, usize), (isize, isize))>
{
    dijkstra_cutoff(grid, advance, start, end, wall, None)
}

pub fn dijkstra_cutoff<T, F>(grid: &Vec<Vec<T>>, advance: &F, start: (usize, usize), end: (usize, usize), wall: T, cost_cutoff: Option<u32>) -> Option<u32> 
where T: Eq + Copy,
F: Fn(u32, (usize, usize), (isize, isize)) -> Vec<(u32, (usize, usize), (isize, isize))>
{
    let size = grid.len();
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(State {
        pos: start,
        d: (0, 1),
        cost: 0,
    });

    while let Some(state) = heap.pop() {
        let p = (state.pos.0 as isize, state.pos.1 as isize);
        if p.0 < 0 || p.1 < 0 || p.0 >= size as isize || p.1 >= size as isize {
            continue;
        }
        if let Some(cutoff) = cost_cutoff {
            if state.cost > cutoff {
                break;
            }
        }
        match state.pos {
            pos if pos == end => return Some(state.cost),
            pos if pos == start => {}, // accommodate edge-case where the start coincides with wall
            pos if grid[pos.0][pos.1] == wall => continue,
            _ => {},
        }
        for (cost, pos, d) in advance(state.cost, state.pos, state.d) {
            if visited.contains(&(pos, d)) {
                continue;
            }
            visited.insert((pos, d));
            heap.push(State { cost, pos, d });
        }
    }
    None
}
