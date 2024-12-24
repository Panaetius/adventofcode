use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
    fs::File,
    hash::{Hash, Hasher},
    io::Read,
    iter::Sum,
};

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
        node_index: &mut HashMap<(usize, usize, bool), NodeIndex>,
    ) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            edges: Vec::new(),
            i,
            j,
            cheat: true,
            is_end,
            cost: usize::MAX,
        });
        node_index.entry((i, j, true)).or_insert(NodeIndex(index));
        self.nodes.push(NodeData {
            edges: Vec::new(),
            i,
            j,
            cheat: false,
            is_end: false,
            cost: usize::MAX,
        });
        node_index
            .entry((i, j, false))
            .or_insert(NodeIndex(index + 1));

        NodeIndex(index + 1)
    }
    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, cost: usize) -> EdgeIndex {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source.0];
        self.edges.push(EdgeData { target, cost });
        node_data.edges.push(EdgeIndex(edge_index));
        EdgeIndex(edge_index)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Hash)]
struct NodeIndex(usize);

#[derive(Clone, Copy, Debug)]
struct EdgeIndex(usize);

#[derive(Clone, Debug)]
struct NodeData {
    pub edges: Vec<EdgeIndex>,
    pub i: usize,
    pub j: usize,
    pub cheat: bool,
    pub is_end: bool,
    pub cost: usize,
}

impl PartialEq for NodeData {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j && self.cheat == other.cheat
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
        self.cheat.hash(state);
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
    //     let contents = "###############
    // #...#...#.....#
    // #.#.#.#.#.###.#
    // #S#...#.#.#...#
    // #######.#.#.###
    // #######.#.#...#
    // #######.#.###.#
    // ###..E#...#...#
    // ###.#######.###
    // #...###...#...#
    // #.#####.#.###.#
    // #.#...#.#.#...#
    // #.#.#.#.#.#.###
    // #...#...#...###
    // ###############";
    let mut start: Option<NodeIndex> = None;
    let mut simple_end: Option<NodeIndex> = None;
    let mut end: Option<NodeIndex> = None;
    let mut graph = Graph::new();
    let mut node_index: HashMap<(usize, usize, bool), NodeIndex> = HashMap::new();
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
                    simple_end = Some(graph.add_node(i, j, true, &mut node_index));
                    end = Some(NodeIndex(simple_end.unwrap().0 - 1));
                }
                _ => continue,
            }
        }
    }
    let mut possible_cheats = HashSet::new();

    //add edges
    for source_index in node_index.values() {
        let source_node = graph.nodes[source_index.0].clone();
        source_node.j.checked_sub(1).and_then(|j| {
            node_index
                .get(&(source_node.i, j, source_node.cheat))
                .map(|&target_index| graph.add_edge(*source_index, target_index, 1))
        });
        source_node.j.checked_add(1).and_then(|j| {
            node_index
                .get(&(source_node.i, j, source_node.cheat))
                .map(|&target_index| graph.add_edge(*source_index, target_index, 1))
        });
        source_node.i.checked_sub(1).and_then(|i| {
            node_index
                .get(&(i, source_node.j, source_node.cheat))
                .map(|&target_index| graph.add_edge(*source_index, target_index, 1))
        });
        source_node.i.checked_add(1).and_then(|i| {
            node_index
                .get(&(i, source_node.j, source_node.cheat))
                .map(|&target_index| graph.add_edge(*source_index, target_index, 1))
        });
        // add cheat connections
        if !source_node.cheat {
            for kk in 2..=20 {
                for jj in 0..=kk {
                    let ii = kk - jj;
                    source_node.i.checked_add(ii).and_then(|i| {
                        source_node.j.checked_add(jj).and_then(|j| {
                            node_index.get(&(i, j, true)).map(|&target_index| {
                                graph.add_edge(*source_index, target_index, kk);
                                possible_cheats.insert((source_index, target_index, kk))
                            })
                        })
                    });
                    source_node.i.checked_add(ii).and_then(|i| {
                        source_node.j.checked_sub(jj).and_then(|j| {
                            node_index.get(&(i, j, true)).map(|&target_index| {
                                graph.add_edge(*source_index, target_index, kk);
                                possible_cheats.insert((source_index, target_index, kk))
                            })
                        })
                    });
                    source_node.i.checked_sub(ii).and_then(|i| {
                        source_node.j.checked_sub(jj).and_then(|j| {
                            node_index.get(&(i, j, true)).map(|&target_index| {
                                graph.add_edge(*source_index, target_index, kk);
                                possible_cheats.insert((source_index, target_index, kk))
                            })
                        })
                    });
                    source_node.i.checked_sub(ii).and_then(|i| {
                        source_node.j.checked_add(jj).and_then(|j| {
                            node_index.get(&(i, j, true)).map(|&target_index| {
                                graph.add_edge(*source_index, target_index, kk);
                                possible_cheats.insert((source_index, target_index, kk))
                            })
                        })
                    });
                }
            }
        }
    }
    println!("floyd?");
    let dist = floyd_warshal(&graph);
    println!("floyd!");
    // let mut cache = HashMap::new();
    // let max_time = get_path(
    //     &mut graph,
    //     &start.unwrap(),
    //     &simple_end.unwrap(),
    //     &mut cache,
    // )
    // .unwrap();
    let max_time = dist[start.unwrap().0][simple_end.unwrap().0];
    println!("max_time:{}", max_time);

    let max_time = max_time - 100;
    println!("max time:{}", max_time);
    let mut tot_cheats = 0;
    println!("num cheats:{}", possible_cheats.len());
    for (i, (source, target, edge_cost)) in possible_cheats.iter().enumerate() {
        if i % 100 == 0 {
            println!("i:{}", i);
        }
        // if let Some(source_cost) = get_path(&mut graph, &start.unwrap(), &source, &mut cache) {
        let source_cost = dist[start.unwrap().0][source.0];
        if source_cost < usize::MAX {
            if source_cost > max_time {
                continue;
            }
            // if let Some(target_cost) = get_path(&mut graph, &target, &end.unwrap(), &mut cache) {
            let target_cost = dist[target.0][end.unwrap().0];
            if target_cost < usize::MAX {
                if source_cost + target_cost + edge_cost <= max_time {
                    tot_cheats += 1;
                }
            }
        }
    }
    println!("{}", tot_cheats);

    Ok(())
}

fn floyd_warshal(graph: &Graph) -> Vec<Vec<usize>> {
    let node_len = graph.nodes.len();
    let mut dist: Vec<Vec<_>> = vec![vec![usize::MAX; node_len]; node_len];

    for source in 0..node_len {
        for edge_index in graph.nodes[source].edges.iter() {
            let (target, cost) = (
                graph.edges[edge_index.0].target.0,
                graph.edges[edge_index.0].cost,
            );
            dist[source][target] = cost;
        }
        dist[source][source] = 0;
    }
    for k in 0..node_len {
        println!("k: {}", k);

        for i in 0..node_len {
            for j in 0..node_len {
                if dist[i][j] > dist[i][k].checked_add(dist[k][j]).unwrap_or(usize::MAX) {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }
    dist
}

fn dfs(
    current: NodeIndex,
    current_cost: usize,
    prev: &mut HashSet<NodeIndex>,
    max: usize,
    graph: &Graph,
    blacklist: &mut HashSet<(NodeIndex, NodeIndex)>,
) -> bool {
    prev.insert(current);
    let is_cheat = graph.nodes[current.0].cheat;
    let mut found = false;
    for edge in graph.nodes[current.0].edges.iter() {
        let (cost, target) = (graph.edges[edge.0].cost, graph.edges[edge.0].target);
        if prev.contains(&target) || blacklist.contains(&(current, target)) {
            continue;
        }
        let new_cost = current_cost + cost;
        if new_cost > max {
            continue;
        }
        let target_is_cheat = graph.nodes[target.0].cheat;
        found = found || dfs(target, new_cost, prev, max, graph, blacklist);
        if found {
            if !is_cheat && target_is_cheat {
                println!("Blacklisted {:?}->{:?}", current, target);
                blacklist.insert((current, target));
            }
            if target_is_cheat {
                break;
            }
        }
    }
    prev.remove(&current);
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
fn get_path(
    graph: &mut Graph,
    start: &NodeIndex,
    end_node: &NodeIndex,
    cache: &mut HashMap<(NodeIndex, NodeIndex), usize>,
) -> Option<usize> {
    //reset all nodes
    if let Some(cost) = cache.get(&(*start, *end_node)) {
        return Some(*cost);
    }
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
        if current == *end_node {
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
            let cost = cur_cost + edge.cost;
            if cost > shortest_cost {
                continue;
            }

            if target.cost > cost {
                unvisited.insert(edge.target.0);
                graph.nodes[edge.target.0].cost = cost;
            }
        }
    }
    if found {
        cache.insert((*start, *end_node), shortest_cost);
        Some(shortest_cost)
    } else {
        None
    }
}
