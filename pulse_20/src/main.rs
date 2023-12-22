use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::combinator::opt;
use nom::error::Error;
use nom::multi::{many1, separated_list1};
use nom::sequence::{pair, separated_pair, terminated};

#[derive(Clone, Debug)]
enum Pulse {
    Low(String, RcNode),
    High(String, RcNode),
}
#[derive(Debug)]
struct Output {
    name: String,
}
#[derive(Debug)]
struct OutputCont(RefCell<Output>);
impl Node for OutputCont {
    fn process(&self, _signal: Pulse) -> Vec<Pulse> {
        Vec::new()
    }

    fn add_edges(&self, _targets: Vec<String>, _nodes: &HashMap<String, RcNode>) {}

    fn incoming_edge(&self, _source: String) {}
}
#[derive(Debug)]
struct Broadcast {
    name: String,
    out_edges: Vec<RcNode>,
}
#[derive(Debug)]
struct BroadcastCont(RefCell<Broadcast>);
type RcNode = Rc<dyn Node>;
impl Node for BroadcastCont {
    fn process(&self, _: Pulse) -> Vec<Pulse> {
        let mut result = Vec::new();
        for edge in &self.0.borrow().out_edges {
            result.push(Pulse::Low(self.0.borrow().name.clone(), edge.clone()));
        }
        result
    }
    fn add_edges(&self, targets: Vec<String>, nodes: &HashMap<String, RcNode>) {
        for target in targets {
            let out = nodes.get(&target).unwrap().clone();
            out.incoming_edge(self.0.borrow().name.clone());
            self.0.borrow_mut().out_edges.push(out);
        }
    }

    fn incoming_edge(&self, _source: String) {}
}
#[derive(Debug)]
struct FlipFlop {
    name: String,
    state: bool,
    out_edges: Vec<RcNode>,
}
#[derive(Debug)]
struct FlipFlopCont(RefCell<FlipFlop>);
impl Node for FlipFlopCont {
    fn process(&self, signal: Pulse) -> Vec<Pulse> {
        match signal {
            Pulse::Low(_, _) => {
                let mut s = self.0.borrow_mut();
                s.state = !s.state;
                if s.state {
                    let mut result = Vec::new();
                    for edge in &s.out_edges {
                        result.push(Pulse::High(s.name.clone(), edge.clone()));
                    }
                    result
                } else {
                    let mut result = Vec::new();
                    for edge in &s.out_edges {
                        result.push(Pulse::Low(s.name.clone(), edge.clone()));
                    }
                    result
                }
            }
            Pulse::High(_, _) => Vec::new(),
        }
    }
    fn add_edges(&self, targets: Vec<String>, nodes: &HashMap<String, RcNode>) {
        for target in targets {
            let out = nodes.get(&target).unwrap().clone();
            let mut s = self.0.borrow_mut();
            out.incoming_edge(s.name.clone());
            s.out_edges.push(out);
        }
    }

    fn incoming_edge(&self, _source: String) {}
}
#[derive(Debug)]
struct Conjunction {
    name: String,
    state: HashMap<String, bool>,
    out_edges: Vec<RcNode>,
}
#[derive(Debug)]
struct ConjunctionCont(RefCell<Conjunction>);
impl Node for ConjunctionCont {
    fn process(&self, signal: Pulse) -> Vec<Pulse> {
        let mut s = self.0.borrow_mut();
        match signal {
            Pulse::Low(n, _) => {
                s.state.entry(n.clone()).and_modify(|v| *v = false);
            }
            Pulse::High(n, _) => {
                s.state.entry(n.clone()).and_modify(|v| *v = true);
            }
        }
        if s.state.values().all(|v| *v) {
            let mut result = Vec::new();
            for edge in &s.out_edges {
                result.push(Pulse::Low(s.name.clone(), edge.clone()));
            }
            result
        } else {
            let mut result = Vec::new();
            for edge in &s.out_edges {
                result.push(Pulse::High(s.name.clone(), edge.clone()));
            }
            result
        }
    }

    fn add_edges(&self, targets: Vec<String>, nodes: &HashMap<String, RcNode>) {
        let mut s = self.0.borrow_mut();
        for target in targets {
            let out = nodes
                .get(&target)
                .expect(format!("not found {}", target).as_str())
                .clone();
            out.incoming_edge(s.name.clone());
            s.out_edges.push(out);
        }
    }

    fn incoming_edge(&self, source: String) {
        let mut s = self.0.borrow_mut();
        s.state.insert(source, false);
    }
}
trait Node: std::fmt::Debug {
    fn process(&self, signal: Pulse) -> Vec<Pulse>;
    fn add_edges(&self, targets: Vec<String>, nodes: &HashMap<String, RcNode>);
    fn incoming_edge(&self, source: String);
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let (_, data) = many1(terminated(
        separated_pair(
            pair(
                opt(alt((tag::<&str, &str, Error<_>>("%"), tag("&")))),
                alpha1,
            ),
            tag(" -> "),
            separated_list1(tag(", "), alpha1),
        ),
        newline,
    ))(contents.as_str())
    .unwrap();

    let mut broadcast: Option<Rc<BroadcastCont>> = None;
    let mut nodes: HashMap<String, RcNode> = HashMap::new();
    println!("{:?}", data);
    let mut existing: HashSet<&str> = HashSet::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for ((kind, name), out) in data.clone() {
        match (kind, name) {
            (None, "broadcaster") => {
                broadcast = Some(Rc::new(BroadcastCont(RefCell::new(Broadcast {
                    name: name.to_string(),
                    out_edges: Vec::new(),
                }))));
                nodes.insert(
                    "broadcaster".to_string(),
                    broadcast.clone().unwrap().clone(),
                );
            }
            (Some("%"), name) => {
                nodes.insert(
                    name.to_string(),
                    Rc::new(FlipFlopCont(RefCell::new(FlipFlop {
                        name: name.to_string(),
                        state: false,
                        out_edges: Vec::new(),
                    }))),
                );
            }
            (Some("&"), name) => {
                nodes.insert(
                    name.to_string(),
                    Rc::new(ConjunctionCont(RefCell::new(Conjunction {
                        name: name.to_string(),
                        state: HashMap::new(),
                        out_edges: Vec::new(),
                    }))),
                );
            }
            _ => unreachable!(),
        }
        seen.insert(name);
        existing.extend(out);
    }
    for missing in existing.difference(&seen) {
        nodes.insert(
            missing.to_string(),
            Rc::new(OutputCont(RefCell::new(Output {
                name: missing.to_string(),
            }))),
        );
    }
    for ((_, name), out) in data {
        let n = nodes.get(&name.to_string()).unwrap().clone();
        n.add_edges(out.iter().map(|s| s.to_string()).collect(), &nodes);
    }
    let mut low_count: usize = 0;
    let mut high_count: usize = 0;
    for _ in 0..1000 {
        let mut signals = VecDeque::from([Pulse::Low(
            "".to_string(),
            broadcast.clone().unwrap().clone(),
        )]);
        while let Some(signal) = signals.pop_front() {
            match signal.clone() {
                Pulse::Low(_, node) => {
                    low_count += 1;
                    for signal in node.process(signal) {
                        signals.push_back(signal);
                    }
                }
                Pulse::High(_, node) => {
                    high_count += 1;
                    for signal in node.process(signal) {
                        signals.push_back(signal);
                    }
                }
            }
        }
    }
    println!("{}:{}", low_count, high_count);
    println!("{}", low_count * high_count);

    Ok(())
}
