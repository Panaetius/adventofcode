use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    hash::{Hash, Hasher},
    io::Read,
    iter::Sum,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}
struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}
impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    fn add_node(
        &mut self,
        i: usize,
        j: usize,
        is_end: bool,
        node_index: &mut HashMap<(usize, usize, Dir), NodeIndex>,
    ) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            edges: Vec::new(),
            i,
            j,
            dir: Dir::Up,
            is_end,
            cost: usize::MAX,
        });
        node_index
            .entry((i, j, Dir::Up))
            .or_insert(NodeIndex(index));
        self.nodes.push(NodeData {
            edges: Vec::new(),
            i,
            j,
            dir: Dir::Left,
            is_end,
            cost: usize::MAX,
        });
        node_index
            .entry((i, j, Dir::Left))
            .or_insert(NodeIndex(index + 1));
        self.nodes.push(NodeData {
            edges: Vec::new(),
            i,
            j,
            dir: Dir::Down,
            is_end,
            cost: usize::MAX,
        });
        node_index
            .entry((i, j, Dir::Down))
            .or_insert(NodeIndex(index + 2));
        self.nodes.push(NodeData {
            edges: Vec::new(),
            i,
            j,
            dir: Dir::Right,
            is_end,
            cost: usize::MAX,
        });
        node_index
            .entry((i, j, Dir::Right))
            .or_insert(NodeIndex(index + 3));

        self.add_edge(NodeIndex(index), NodeIndex(index + 3), 1000);
        self.add_edge(NodeIndex(index), NodeIndex(index + 1), 1000);

        self.add_edge(NodeIndex(index + 1), NodeIndex(index), 1000);
        self.add_edge(NodeIndex(index + 1), NodeIndex(index + 2), 1000);

        self.add_edge(NodeIndex(index + 2), NodeIndex(index + 1), 1000);
        self.add_edge(NodeIndex(index + 2), NodeIndex(index + 3), 1000);

        self.add_edge(NodeIndex(index + 3), NodeIndex(index + 2), 1000);
        self.add_edge(NodeIndex(index + 3), NodeIndex(index), 1000);

        NodeIndex(index + 3)
    }
    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, cost: usize) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source.0];
        self.edges.push(EdgeData { target, cost });
        node_data.edges.push(EdgeIndex(edge_index));
    }
}
#[derive(Clone, Copy, Debug)]
struct NodeIndex(usize);

#[derive(Clone, Copy, Debug)]
struct EdgeIndex(usize);

#[derive(Clone, Debug)]
struct NodeData {
    pub edges: Vec<EdgeIndex>,
    pub i: usize,
    pub j: usize,
    pub dir: Dir,
    pub is_end: bool,
    pub cost: usize,
}

impl PartialEq for NodeData {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j && self.dir == other.dir
    }
}

impl Eq for NodeData {}

impl PartialOrd for NodeData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.j.cmp(&other.j))
            .then_with(|| self.i.cmp(&other.i))
    }
}

impl Hash for NodeData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.i.hash(state);
        self.j.hash(state);
        self.dir.hash(state);
    }
}

struct EdgeData {
    pub target: NodeIndex,
    pub cost: usize,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //     let contents = "###############
    // #.......#....E#
    // #.#.###.#.###.#
    // #.....#.#...#.#
    // #.###.#####.#.#
    // #.#.#.......#.#
    // #.#.#####.###.#
    // #...........#.#
    // ###.#.#####.#.#
    // #...#.....#.#.#
    // #.#.#.###.#.#.#
    // #.....#...#.#.#
    // #.###.#.#.#.#.#
    // #S..#.....#...#
    // ###############";
    //     let contents = "#################
    // #...#...#...#..E#
    // #.#.#.#.#.#.#.#.#
    // #.#.#.#...#...#.#
    // #.#.#.#.###.#.#.#
    // #...#.#.#.....#.#
    // #.#.#.#.#.#####.#
    // #.#...#.#.#.....#
    // #.#.#####.#.###.#
    // #.#.#.......#...#
    // #.#.###.#####.###
    // #.#.#...#.....#.#
    // #.#.#.#####.###.#
    // #.#.#.........#.#
    // #.#.#.#########.#
    // #S#.............#
    // #################";
    let mut start: Option<NodeIndex> = None;
    let mut graph = Graph::new();
    let mut node_index: HashMap<(usize, usize, Dir), NodeIndex> = HashMap::new();
    for (j, l) in contents.trim().split("\n").enumerate() {
        for (i, c) in l.chars().enumerate() {
            match c {
                '.' => {
                    graph.add_node(i, j, false, &mut node_index);
                }
                'S' => {
                    start = Some(graph.add_node(i, j, false, &mut node_index));
                }
                'E' => {
                    graph.add_node(i, j, true, &mut node_index);
                }
                _ => continue,
            }
        }
    }

    //add edges
    for source_index in node_index.values() {
        let node = graph.nodes[source_index.0].clone();
        let _ = match node.dir {
            Dir::Up => node.j.checked_sub(1).and_then(|j| {
                node_index
                    .get(&(node.i, j, node.dir))
                    .map(|&target_index| graph.add_edge(*source_index, target_index, 1))
            }),
            Dir::Left => node.i.checked_sub(1).and_then(|i| {
                node_index
                    .get(&(i, node.j, node.dir))
                    .map(|&target_index| graph.add_edge(*source_index, target_index, 1))
            }),
            Dir::Down => node.j.checked_add(1).and_then(|j| {
                node_index
                    .get(&(node.i, j, node.dir))
                    .map(|&target_index| graph.add_edge(*source_index, target_index, 1))
            }),
            Dir::Right => node.i.checked_add(1).and_then(|i| {
                node_index
                    .get(&(i, node.j, node.dir))
                    .map(|&target_index| graph.add_edge(*source_index, target_index, 1))
            }),
        };
    }
    let start = start.unwrap();
    graph.nodes[start.0].cost = 0;

    let mut unvisited = HashSet::new();
    unvisited.insert(start.0);
    let mut visited = HashSet::new();
    let mut tot_cost = 0;
    let mut pathmap = HashMap::new();
    let mut end = 0;
    let mut shortest_cost = usize::MAX;
    let mut steps = 0;
    while let Some(current) = get_lowest(&graph, &unvisited) {
        steps += 1;
        let current_node = &graph.nodes[current.0];
        let curi = (current_node.i, current_node.j, current_node.dir);
        // println!("{:?}", current_node);
        let cur_cost = current_node.cost;
        unvisited.remove(&current.0);
        visited.insert(current.0);
        if current_node.is_end {
            tot_cost = cur_cost;
            shortest_cost = cur_cost;
            end = current.0;
            continue;
        }
        let edges = current_node.edges.clone();
        for edge_index in edges.iter() {
            let edge = &graph.edges[edge_index.0];
            let target = &graph.nodes[edge.target.0];
            // if visited.contains(&edge.target.0) {
            //     continue;
            // }
            let cost = cur_cost + edge.cost;
            if cost > shortest_cost {
                continue;
            }
            // if (target.i, target.j) == (15, 7) {
            //     println!(
            //         "visited 15,7: {} : {:?}<-{:?}, {}",
            //         edge.target.0, target, curi, cost
            //     );
            // }

            if target.cost > cost {
                unvisited.insert(edge.target.0);
                graph.nodes[edge.target.0].cost = cost;
                pathmap
                    .entry(edge.target.0)
                    .and_modify(|v| *v = vec![current.0])
                    .or_insert(vec![current.0]);
            } else if target.cost == cost {
                pathmap
                    .entry(edge.target.0)
                    .and_modify(|v| v.push(current.0))
                    .or_insert(vec![current.0]);
            }
        }
    }
    println!("{}", steps);
    println!("{}", tot_cost);
    println!("{}", pathmap.len());
    let mut pathnodes = HashSet::new();
    pathnodes.insert(&end);
    println!("{:?}", pathmap[&256]);
    let mut stack = vec![end];
    while let Some(current) = stack.pop() {
        let next = pathmap.get(&current);
        // println!("{:?}", next);
        if let Some(next) = next {
            for n in next {
                if !pathnodes.contains(&n) {
                    stack.push(*n);
                    pathnodes.insert(n);
                }
            }
        } else {
            println!("no next for {}:{:?}", current, graph.nodes[current]);
        }
    }
    let pathnodes: HashSet<_> = pathnodes
        .iter()
        .map(|n| (graph.nodes[**n].i, graph.nodes[**n].j))
        .collect();
    println!("count:{}", pathnodes.len());
    print_nodes(&contents, graph, pathnodes);

    Ok(())
}

fn get_lowest(graph: &Graph, unvisited: &HashSet<usize>) -> Option<NodeIndex> {
    let mut lowest = usize::MAX;
    let mut index = None;
    for n in unvisited {
        let cost = graph.nodes[*n].cost;
        if cost < lowest {
            index = Some(NodeIndex(*n));
            lowest = cost;
        }
    }
    index
}

fn print_nodes(contents: &str, graph: Graph, nodes: HashSet<(usize, usize)>) -> () {
    let content = contents
        .trim()
        .split("\n")
        .enumerate()
        .map(|(j, l)| {
            l.chars()
                .enumerate()
                .map(|(i, c)| if nodes.contains(&(i, j)) { 'O' } else { c })
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", content);
}
