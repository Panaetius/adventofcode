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
#[derive(Debug)]
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
        node_index: &mut HashMap<(usize, usize), NodeIndex>,
    ) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            edges: Vec::new(),
            i,
            j,
            is_end,
            cost: usize::MAX,
        });
        node_index.entry((i, j)).or_insert(NodeIndex(index));

        NodeIndex(index)
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
    pub is_end: bool,
    pub cost: usize,
}

impl PartialEq for NodeData {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j
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
    }
}

#[derive(Debug)]
struct EdgeData {
    pub target: NodeIndex,
    pub cost: usize,
}
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: Vec<_> = contents
        .trim()
        .split("\n")
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
        .collect();
    let (width, height) = (70, 70);
    let mut map = vec![vec![true; width + 1]; height + 1];
    for (x, y) in data.iter().take(1024) {
        map[*y][*x] = false;
    }

    let mut graph = Graph::new();
    let mut node_index: HashMap<(usize, usize), NodeIndex> = HashMap::new();

    for j in 0..height + 1 {
        for i in 0..width + 1 {
            if map[j][i] {
                graph.add_node(i, j, j == height && i == width, &mut node_index);
            }
        }
    }
    for source_index in node_index.values() {
        let source_node = graph.nodes[source_index.0].clone();
        source_node.j.checked_sub(1).and_then(|j| {
            node_index
                .get(&(source_node.i, j))
                .map(|&target_index| graph.add_edge(*source_index, target_index, 1))
        });
        source_node.j.checked_add(1).and_then(|j| {
            node_index
                .get(&(source_node.i, j))
                .map(|&target_index| graph.add_edge(*source_index, target_index, 1))
        });
        source_node.i.checked_sub(1).and_then(|i| {
            node_index
                .get(&(i, source_node.j))
                .map(|&target_index| graph.add_edge(*source_index, target_index, 1))
        });
        source_node.i.checked_add(1).and_then(|i| {
            node_index
                .get(&(i, source_node.j))
                .map(|&target_index| graph.add_edge(*source_index, target_index, 1))
        });
    }
    let start = node_index.get(&(0, 0)).unwrap();
    let mut blacklist = HashSet::new();
    for (x, y) in data.iter().skip(1024) {
        println!("({},{})", x, y);
        blacklist.insert((x, y));
        let shortest_cost = get_path(&mut graph, start, &blacklist);
        if !shortest_cost {
            println!("{},{}", x, y);
            break;
        }
    }
    Ok(())
}

fn get_path(graph: &mut Graph, start: &NodeIndex, blacklist: &HashSet<(&usize, &usize)>) -> bool {
    //reset all nodes
    for i in 0..graph.nodes.len() {
        graph.nodes[i].cost = usize::MAX;
    }

    graph.nodes[start.0].cost = 0;

    let mut unvisited = HashSet::new();
    unvisited.insert(start.0);
    let mut visited = HashSet::new();
    let mut tot_cost = 0;
    // let mut pathmap = HashMap::new();
    let mut end = 0;
    let mut shortest_cost = usize::MAX;
    let mut steps = 0;
    let mut found = false;
    while let Some(current) = get_lowest(&graph, &unvisited) {
        steps += 1;
        let current_node = &graph.nodes[current.0];
        let curi = (current_node.i, current_node.j);
        // println!("{:?}", current_node);
        let cur_cost = current_node.cost;
        unvisited.remove(&current.0);
        visited.insert(current.0);
        if current_node.is_end {
            tot_cost = cur_cost;
            shortest_cost = cur_cost;
            end = current.0;
            found = true;
            break;
        }
        let edges = current_node.edges.clone();
        for edge_index in edges.iter() {
            let edge = &graph.edges[edge_index.0];
            let target = &graph.nodes[edge.target.0];
            if blacklist.contains(&(&target.i, &target.j)) {
                continue;
            }
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
                // pathmap
                //     .entry(edge.target.0)
                //     .and_modify(|v| *v = vec![current.0])
                //     .or_insert(vec![current.0]);
            }
            //  else if target.cost == cost {
            //     pathmap
            //         .entry(edge.target.0)
            //         .and_modify(|v| v.push(current.0))
            //         .or_insert(vec![current.0]);
            // }
        }
    }
    found
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
