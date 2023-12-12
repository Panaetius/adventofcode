use core::panic;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Cards {
    J,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    Q,
    K,
    A,
}
impl From<char> for Cards {
    fn from(item: char) -> Self {
        match item {
            'J' => Cards::J,
            '2' => Cards::C2,
            '3' => Cards::C3,
            '4' => Cards::C4,
            '5' => Cards::C5,
            '6' => Cards::C6,
            '7' => Cards::C7,
            '8' => Cards::C8,
            '9' => Cards::C9,
            'T' => Cards::T,
            'Q' => Cards::Q,
            'K' => Cards::K,
            'A' => Cards::A,
            _ => panic!("unknown card"),
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    High,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourKind,
    FiveKind,
}
impl From<&[Cards; 5]> for Kind {
    fn from(item: &[Cards; 5]) -> Self {
        let mut binding = item.iter().fold(HashMap::new(), |mut map, val| {
            map.entry(val).and_modify(|f| *f += 1).or_insert(1);
            map
        });
        let jokers = binding.remove(&Cards::J).unwrap_or(0);
        let mut counts: Vec<_> = binding.values().sorted().rev().collect();
        if counts.len() == 0 {
            counts.push(&0);
        }
        if counts.get(0).and_then(|&v| Some(v + jokers)) == Some(5) {
            Kind::FiveKind
        } else if counts.get(0).and_then(|&v| Some(v + jokers)) == Some(4) {
            Kind::FourKind
        } else if counts.get(0).and_then(|&v| Some(v + jokers)) == Some(3)
            && counts.get(1) == Some(&&2)
        {
            Kind::FullHouse
        } else if counts.get(0).and_then(|&v| Some(v + jokers)) == Some(3) {
            Kind::ThreeOfKind
        } else if counts.get(0).and_then(|&v| Some(v + jokers)) == Some(2)
            && counts.get(1) == Some(&&2)
        {
            Kind::TwoPair
        } else if counts.get(0).and_then(|&v| Some(v + jokers)) == Some(2) {
            Kind::OnePair
        } else {
            Kind::High
        }
    }
}
#[derive(Debug)]
struct Hand {
    cards: [Cards; 5],
    kind: Kind,
}
impl From<&str> for Hand {
    fn from(item: &str) -> Self {
        let hand: [Cards; 5] = item
            .chars()
            .map(Cards::from)
            .collect::<Vec<Cards>>()
            .try_into()
            .unwrap();
        let kind: Kind = (&hand).into();
        Self {
            cards: hand,
            kind: kind,
        }
    }
}
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split("\n");
    let mut hands: Vec<(Hand, u32)> = lines
        .filter_map(|l| l.split_once(" "))
        .map(|(c, s)| (Hand::from(c), s.parse().unwrap()))
        .collect();
    hands.sort_by(|a, b| {
        a.0.kind
            .cmp(&b.0.kind)
            .then_with(|| a.0.cards.cmp(&b.0.cards))
    });
    println!("{:?}", hands);
    let result = hands
        .iter()
        .enumerate()
        .fold(0u32, |acc, (i, val)| acc + (i as u32 + 1) * val.1);
    println!("{:?}", result);

    Ok(())
}
