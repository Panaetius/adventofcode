use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, tuple};
use nom::IResult;
#[derive(Debug)]
struct RangeMapEntry {
    source_start: u64,
    target_start: u64,
    length: u64,
}
impl RangeMapEntry {
    fn transform(&self, val: u64) -> Option<u64> {
        if val >= self.source_start && val < self.source_start + self.length {
            Some(val - self.source_start + self.target_start)
        } else {
            None
        }
    }

    fn modify_with_range(&self, other: &RangeMapEntry) -> Option<Vec<RangeMapEntry>> {
        if self.source_start + self.length < other.target_start
            || self.source_start > other.target_start + other.length
        {
            return None;
        } else {
            let result: Vec<RangeMapEntry> = Vec::new();
            if self.source_start < other.target_start {
                result.push(RangeMapEntry {
                    source_start: self.source_start,
                    target_start: self.target_start,
                    length: other.target_start - self.source_start,
                });
            } else {
                result.push()
            }
        }
    }
}

#[derive(Debug)]
struct Data {
    seeds: Vec<(u64, u64)>,
    seeds_to_soil: Vec<RangeMapEntry>,
    soil_to_fert: Vec<RangeMapEntry>,
    fert_to_water: Vec<RangeMapEntry>,
    water_to_light: Vec<RangeMapEntry>,
    light_to_temp: Vec<RangeMapEntry>,
    temp_to_humid: Vec<RangeMapEntry>,
    humid_to_loc: Vec<RangeMapEntry>,
}
fn seeds(i: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (i, _) = tag("seeds: ")(i)?;
    let (i, numbers) = separated_list1(space1, separated_pair(digit1, space1, digit1))(i)?;
    let numbers: Vec<_> = numbers
        .iter()
        .map(|(n, n2)| (n.parse::<u64>().unwrap(), n2.parse::<u64>().unwrap()))
        .collect();
    let (i, _) = tag("\n")(i)?;
    Ok((i, numbers))
}
fn range(i: &str) -> IResult<&str, RangeMapEntry> {
    let (i, (dest, _, src, _, len)) = tuple((digit1, space1, digit1, space1, digit1))(i)?;
    let (i, _) = tag("\n")(i)?;
    Ok((
        i,
        RangeMapEntry {
            source_start: src.parse().unwrap(),
            target_start: dest.parse().unwrap(),
            length: len.parse().unwrap(),
        },
    ))
}
fn range_block(i: &str) -> IResult<&str, Vec<RangeMapEntry>> {
    let (i, ranges) = many1(range)(i)?;
    let (i, _) = many1(tag("\n"))(i)?;
    Ok((i, ranges))
}
fn parse_blocks(i: &str) -> IResult<&str, Data> {
    let (i, seeds) = seeds(i)?;
    let (i, _) = many1(tag("\n"))(i)?;
    let (i, _) = tag("seed-to-soil map:\n")(i)?;
    let (i, ss) = range_block(i)?;
    let (i, _) = tag("soil-to-fertilizer map:\n")(i)?;
    let (i, sf) = range_block(i)?;
    let (i, _) = tag("fertilizer-to-water map:\n")(i)?;
    let (i, fw) = range_block(i)?;
    let (i, _) = tag("water-to-light map:\n")(i)?;
    let (i, wl) = range_block(i)?;
    let (i, _) = tag("light-to-temperature map:\n")(i)?;
    let (i, lt) = range_block(i)?;
    let (i, _) = tag("temperature-to-humidity map:\n")(i)?;
    let (i, th) = range_block(i)?;
    let (i, _) = tag("humidity-to-location map:\n")(i)?;
    let (i, hl) = range_block(i)?;
    Ok((
        i,
        Data {
            seeds: seeds,
            seeds_to_soil: ss,
            soil_to_fert: sf,
            fert_to_water: fw,
            water_to_light: wl,
            light_to_temp: lt,
            temp_to_humid: th,
            humid_to_loc: hl,
        },
    ))
}

fn parse(input: impl AsRef<str>) -> Result<Data, ()> {
    match parse_blocks(input.as_ref()) {
        Ok((r, t)) => {
            if r.is_empty() {
                Ok(t)
            } else {
                println!("not empty at end:{}", r);
                Err(())
            }
        }
        Err(e) => {
            println!("error: {:#?}", e);
            Err(())
        }
    }
}
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // println!("{}", contents);
    let data = parse(contents).unwrap();
    println!("{:?}", data);
    // println("{:?}", m6);

    // println!("{:?}", data);
    let results = data
        .seeds
        .iter()
        .flat_map(|&(s, l)| s..(s + l))
        .map(|n| *m6.get(&n).unwrap_or(&n))
        .min()
        .unwrap();
    // .map(|s| {
    //     data.seeds_to_soil
    //         .iter()
    //         .find_map(|i| i.transform(s))
    //         .unwrap_or(s)
    // })
    // .map(|s| {
    //     data.soil_to_fert
    //         .iter()
    //         .find_map(|i| i.transform(s))
    //         .unwrap_or(s)
    // })
    // .map(|s| {
    //     data.fert_to_water
    //         .iter()
    //         .find_map(|i| i.transform(s))
    //         .unwrap_or(s)
    // })
    // .map(|s| {
    //     data.water_to_light
    //         .iter()
    //         .find_map(|i| i.transform(s))
    //         .unwrap_or(s)
    // })
    // .map(|s| {
    //     data.light_to_temp
    //         .iter()
    //         .find_map(|i| i.transform(s))
    //         .unwrap_or(s)
    // })
    // .map(|s| {
    //     data.temp_to_humid
    //         .iter()
    //         .find_map(|i| i.transform(s))
    //         .unwrap_or(s)
    // })
    // .map(|s| {
    //     data.humid_to_loc
    //         .iter()
    //         .find_map(|i| i.transform(s))
    //         .unwrap_or(s)
    // })
    // .min()
    // .unwrap();
    println!("{}", results);
    Ok(())
}
