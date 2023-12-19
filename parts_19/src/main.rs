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
    let mut accepted: Vec<HashMap<&str, usize>> = Vec::new();
    for part in parts {
        let mut curr = "in";
        while true {
            match rules[curr].process(&part) {
                Action::Accepted => {
                    accepted.push(part);
                    break;
                }
                Action::Rejected => {
                    break;
                }
                Action::Forward(s) => {
                    curr = s;
                }
            }
        }
    }
    let result: usize = accepted.iter().map(|p| p.values().sum::<usize>()).sum();
    println!("{:?}", result);

    Ok(())
}
