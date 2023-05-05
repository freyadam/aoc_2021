use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

/// vertices are in this case denoted by integers rather than by strings
type Vertex = u16;

///
struct Graph {
    /// a vector of vertices reachable for each existing vertex
    /// (oriented edges)
    edges: HashMap<Vertex, Vec<Vertex>>,
    /// can a particular vertex be revisited without limitations?
    /// (large caves)
    is_revisitable: HashMap<Vertex, bool>,
    /// how many times was given vertex visited on the current path
    open_count: HashMap<Vertex, u16>,
}

const START_VERTEX: u16 = 0;
const END_VERTEX: u16 = 1;

fn init_graph(s: String) -> Graph {
    let mut edges: HashMap<Vertex, Vec<Vertex>> = HashMap::new();
    let mut is_revisitable: HashMap<Vertex, bool> = HashMap::new();
    let mut open_count: HashMap<Vertex, u16> = HashMap::new();

    let mut vertex_names: HashMap<&str, Vertex> =
        HashMap::from([("start", START_VERTEX), ("end", END_VERTEX)]);
    let mut a: u16;
    let mut b: u16;

    // process each defined edge
    let mut v = s.lines().map(|s| s.split("-").collect_tuple().unwrap());
    while let Some((sa, sb)) = v.next() {
        // add a new vertex identifier if necessary
        if !vertex_names.contains_key(sa) {
            vertex_names.insert(sa, vertex_names.len() as Vertex);
        }
        a = *vertex_names.get(sa).unwrap();

        if !vertex_names.contains_key(sb) {
            vertex_names.insert(sb, vertex_names.len() as Vertex);
        }
        b = *vertex_names.get(sb).unwrap();

        // extend the `is_revisitable` map
        if !is_revisitable.contains_key(&a) {
            is_revisitable.insert(a, sa.chars().map(|c| c.is_uppercase()).all(|b| b));
        }
        if !is_revisitable.contains_key(&b) {
            is_revisitable.insert(b, sb.chars().map(|c| c.is_uppercase()).all(|b| b));
        }

        // extend the `open_count` map
        if !open_count.contains_key(&a) {
            open_count.insert(a, 0);
        }
        if !open_count.contains_key(&b) {
            open_count.insert(b, 0);
        }
        // insert an edge a->b
        if !edges.contains_key(&a) {
            edges.insert(a, Vec::new());
        }
        edges.get_mut(&a).unwrap().push(b);

        // insert an edge b->a
        if !edges.contains_key(&b) {
            edges.insert(b, Vec::new());
        }
        edges.get_mut(&b).unwrap().push(a);
    }

    // return the starting position of a graph before a DFS is run
    // as defined by the input
    Graph {
        edges: edges,
        is_revisitable: is_revisitable,
        open_count: open_count,
    }
}

fn can_visit_ex1(g: &Graph, v: Vertex) -> bool {
    // a cave is either larger or it was not yet visited on the current DFS path
    *g.is_revisitable.get(&v).unwrap() || *g.open_count.get(&v).unwrap() == 0
}

fn can_visit_ex2(g: &Graph, v: Vertex) -> bool {
    // either a cave is large and then it can be visited as many times as possible ...
    if *g.is_revisitable.get(&v).unwrap() {
        return true;
    }

    let mut small_cave_visited: bool = false;
    for (vtx, _) in g.is_revisitable.iter().filter(|(_, val)| !(**val)) {
        small_cave_visited |= *g.open_count.get(&vtx).unwrap() == 2;
    }

    // or it is a starting vertex, and then it cannot be visited again ever ...
    let cutoff = if v == START_VERTEX {
        0
    // or it is a small cave but a different small cave was already visited,
    // so it can be visited only for the first time ...
    } else if small_cave_visited {
        0
    // or it is a small cave that can be visited even a second time
    } else {
        1
    };
    *g.open_count.get(&v).unwrap() <= cutoff
}

fn dfs(g: &mut Graph, v: Vertex, can_visit_fn: fn(&Graph, Vertex) -> bool) -> u32 {
    let mut sum: u32 = 0;

    // return straight away if the end vertex was reached ...
    if v == END_VERTEX {
        return 1;
    }

    // otherwise iterate over all vertices that can be visited and recursively DFS
    // on each of them
    let open_count_v: u16 = *g.open_count.get(&v).unwrap();
    g.open_count.insert(v, open_count_v + 1);
    let vertices = g.edges.get(&v).unwrap().clone();
    for w in vertices.iter() {
        if !can_visit_fn(g, *w) {
            continue;
        }
        sum += dfs(g, *w, can_visit_fn);
    }
    g.open_count.insert(v, open_count_v);

    sum
}

fn count_paths(mut g: Graph, can_visit_fn: fn(&Graph, Vertex) -> bool) -> u32 {
    dfs(&mut g, START_VERTEX, can_visit_fn)
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day12.txt").expect("Could not read file");
    let g = init_graph(input_str);

    count_paths(g, can_visit_ex1).to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day12.txt").expect("Could not read file");
    let g = init_graph(input_str);

    count_paths(g, can_visit_ex2).to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "4773");
        assert_eq!(ex2(), "116985");
    }
}
