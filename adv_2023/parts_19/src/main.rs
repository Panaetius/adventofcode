use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, newline};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, pair, separated_pair, terminated, tuple};
use nom::IResult;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Action<'a> {
    Accepted,
    Rejected,
    Forward(&'a str),
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Ruleset<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    end: Action<'a>,
}
impl<'a> Ruleset<'a> {
    fn process(&self, part: &HashMap<&str, usize>) -> Action<'a> {
        for rule in self.rules.clone() {
            if let Some(a) = rule.process(&part) {
                return a;
            }
        }
        self.end
    }
    fn range<'b>(
        &'b self,
        rangeset: HashMap<&'b str, Range>,
    ) -> (Vec<(&str, HashMap<&str, Range>)>, Vec<HashMap<&str, Range>>) {
        let mut follow = Vec::new();
        let mut acc = Vec::new();
        let mut rangeset = rangeset;
        for rule in self.rules.clone() {
            let (a, r) = rule.disc.split(rangeset.get(rule.label).unwrap().clone());
            if let Some(r) = a {
                let mut n = rangeset.clone();
                n.entry(rule.label).and_modify(|v| *v = r);
                match rule.target {
                    Action::Accepted => acc.push(n.clone()),
                    Action::Rejected => {}
                    Action::Forward(s) => follow.push((s, n.clone())),
                }
            }
            if let Some(r) = r {
                let mut n = rangeset.clone();
                n.entry(rule.label).and_modify(|v| *v = r);
                rangeset = n;
            }
        }
        match self.end {
            Action::Accepted => acc.push(rangeset.clone()),
            Action::Rejected => {}
            Action::Forward(s) => follow.push((s, rangeset.clone())),
        }
        (follow, acc)
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Discriminator {
    LT(usize),
    GT(usize),
}
impl Discriminator {
    fn apply(&self, value: usize) -> bool {
        match self {
            Discriminator::LT(v) => value < *v,
            Discriminator::GT(v) => value > *v,
        }
    }
    fn split(&self, range: Range) -> (Option<Range>, Option<Range>) {
        match self {
            Discriminator::LT(v) => {
                if *v <= range.start {
                    (None, Some(range))
                } else if range.end < *v {
                    (Some(range), None)
                } else {
                    (
                        Some(Range {
                            start: range.start,
                            end: v - 1,
                        }),
                        Some(Range {
                            start: *v,
                            end: range.end,
                        }),
                    )
                }
            }
            Discriminator::GT(v) => {
                if *v < range.start {
                    (Some(range), None)
                } else if range.end <= *v {
                    (None, Some(range))
                } else {
                    (
                        Some(Range {
                            start: v + 1,
                            end: range.end,
                        }),
                        Some(Range {
                            start: range.start,
                            end: *v,
                        }),
                    )
                }
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Rule<'a> {
    label: &'a str,
    disc: Discriminator,
    target: Action<'a>,
}
impl<'a> Rule<'a> {
    fn process(&self, part: &HashMap<&str, usize>) -> Option<Action<'a>> {
        if self.disc.apply(part[self.label]) {
            Some(self.target)
        } else {
            None
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Range {
    start: usize,
    end: usize,
}
impl Range {
    fn intersect(&self, other: Range) -> Option<Range> {
        if self.end < other.start || self.start > other.end {
            None
        } else {
            Some(Range {
                start: max(self.start, other.start),
                end: min(self.end, other.end),
            })
        }
    }
    fn size(&self) -> usize {
        self.end - self.start + 1
    }
    fn add(&self, other: Range) -> usize {
        self.size() + other.size() - self.intersect(other).map(|i| i.size()).unwrap_or(0)
    }
}
fn rules(i: &str) -> IResult<&str, HashMap<&str, Ruleset>> {
    let (i, rules) = many1(terminated(
        tuple((
            alpha1,
            delimited(
                tag("{"),
                separated_pair(
                    separated_list1(
                        tag(","),
                        tuple((alpha1, alt((tag("<"), tag(">"))), digit1, tag(":"), alpha1)),
                    ),
                    tag(","),
                    alpha1,
                ),
                tag("}"),
            ),
        )),
        newline,
    ))(i)?;
    Ok((
        i,
        rules
            .iter()
            .map(|rs| {
                (
                    rs.0,
                    Ruleset {
                        name: rs.0,
                        rules: rs
                            .1
                             .0
                            .iter()
                            .map(|r| Rule {
                                label: r.0,
                                disc: match r.1 {
                                    "<" => Discriminator::LT(r.2.parse().unwrap()),
                                    ">" => Discriminator::GT(r.2.parse().unwrap()),
                                    _ => unreachable!(),
                                },
                                target: match r.4 {
                                    "A" => Action::Accepted,
                                    "R" => Action::Rejected,
                                    s => Action::Forward(s),
                                },
                            })
                            .collect(),
                        end: match rs.1 .1 {
                            "A" => Action::Accepted,
                            "R" => Action::Rejected,
                            s => Action::Forward(s),
                        },
                    },
                )
            })
            .collect(),
    ))
}
fn parts(i: &str) -> IResult<&str, Vec<HashMap<&str, usize>>> {
    let (i, parts) = many1(terminated(
        delimited(
            tag("{"),
            separated_list1(tag(","), separated_pair(alpha1, tag("="), digit1)),
            tag("}"),
        ),
        newline,
    ))(i)?;
    Ok((
        i,
        parts
            .iter()
            .map(|p| {
                p.iter()
                    .map(|(k, v)| (*k, v.parse::<usize>().unwrap()))
                    .collect::<HashMap<&str, usize>>()
            })
            .collect(),
    ))
}
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let (i, (rules, parts)) = separated_pair(rules, newline, parts)(&contents).unwrap();
    println!("{:?}", rules);
    println!("{:?}", parts);
    // let mut accepted: Vec<HashMap<&str, usize>> = Vec::new();
    // for part in parts {
    //     let mut curr = "in";
    //     while true {
    //         match rules[curr].process(&part) {
    //             Action::Accepted => {
    //                 accepted.push(part);
    //                 break;
    //             }
    //             Action::Rejected => {
    //                 break;
    //             }
    //             Action::Forward(s) => {
    //                 curr = s;
    //             }
    //         }
    //     }
    // }
    // let result: usize = accepted.iter().map(|p| p.values().sum::<usize>()).sum();
    // println!("{:?}", result);
    let mut accepted: Vec<HashMap<&str, Range>> = Vec::new();
    let mut stack = vec![(
        "in",
        HashMap::from([
            (
                "x",
                Range {
                    start: 1,
                    end: 4000,
                },
            ),
            (
                "m",
                Range {
                    start: 1,
                    end: 4000,
                },
            ),
            (
                "a",
                Range {
                    start: 1,
                    end: 4000,
                },
            ),
            (
                "s",
                Range {
                    start: 1,
                    end: 4000,
                },
            ),
        ]),
    )];
    while stack.len() > 0 {
        let cur = stack.pop().unwrap();
        let rule = rules.get(&cur.0).unwrap();
        let (follow, acc) = rule.range(cur.1);
        accepted.extend(acc);
        stack.extend(follow);
    }
    println!("{:?}", accepted);
    let mut sum: usize = accepted
        .iter()
        .map(|v| v.values().map(|i| i.size()).fold(1, |a, c| a * c))
        .sum();
    sum -= accepted
        .iter()
        .combinations(2)
        .map(|v| {
            let a = v[0];
            let b = v[1];
            a.iter()
                .map(|(k, v)| {
                    v.intersect(b[k].clone())
                        .and_then(|i| Some(i.size()))
                        .unwrap_or(0)
                })
                .fold(1, |a, c| a * c)
        })
        .sum::<usize>();
    println!("{}", sum);

    Ok(())
}
