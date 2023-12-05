#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs::read_to_string, ops::Range, str::Lines};

fn main() {
    // println!("==== Part 1 output ====");
    // part_1(read_to_string("day05/part1.input.txt").unwrap().lines());
    println!("\n==== Part 2 output ====");
    part_2(read_to_string("day05/part1.input.txt").unwrap().lines());
}

#[derive(Debug)]
struct RangeMap {
    dst_start: usize,
    src_start: usize,
    length: usize,
}

impl RangeMap {
    fn src_range(&self) -> Range<usize> {
        self.src_start..(self.src_start + self.length)
    }

    fn resolve(&self, n: usize) -> usize {
        return if self.src_range().contains(&n) {
            self.dst_start + n - self.src_start
        } else {
            n
        };
    }
}

#[derive(Debug)]
struct RangeMapCollection {
    range_maps: Vec<RangeMap>,
}

impl RangeMapCollection {
    fn new() -> Self {
        Self {
            range_maps: Vec::new(),
        }
    }

    fn add_range_map(&mut self, range_map: RangeMap) {
        self.range_maps.push(range_map);
    }

    fn resolve(&self, n: usize) -> usize {
        let range_map = self
            .range_maps
            .iter()
            .find(|rm| rm.src_range().contains(&n));
        return match range_map {
            Some(rm) => rm.resolve(n),
            None => n,
        };
    }
}

fn part_1(lines: Lines) {
    let lines_vec = lines.collect::<Vec<&str>>();
    let mut parts = lines_vec.split(|line| line.is_empty());

    let seeds_str = parts.next().unwrap()[0];
    let seeds = seeds_str
        .replace("seeds: ", "")
        .split(' ')
        .map(|num_str| num_str.parse::<usize>())
        .filter_map(|num| num.ok())
        .collect::<Vec<usize>>();
    println!("seeds: {seeds:?}");

    // seed-to-soil map:
    // soil-to-fertilizer map:
    // ...
    let mut range_map_collections: Vec<RangeMapCollection> = Vec::new();
    while let Some(mapping_step_str) = parts.next() {
        let mapping_steps = mapping_step_str.iter().skip(1);
        let range_maps = mapping_steps
            .map(|ms| {
                ms.split(' ')
                    .map(|num_str| num_str.parse::<usize>())
                    .filter_map(|num| num.ok())
                    .collect::<Vec<usize>>()
            })
            .map(|range_map_parts| RangeMap {
                dst_start: range_map_parts[0],
                src_start: range_map_parts[1],
                length: range_map_parts[2],
            });
        let mut range_map_collection = RangeMapCollection::new();
        range_maps.for_each(|rm| range_map_collection.add_range_map(rm));
        range_map_collections.push(range_map_collection);
    }

    let lowest_location = seeds
        .iter()
        .map(|seed| {
            range_map_collections
                .iter()
                .fold(*seed, |acc, rmc| rmc.resolve(acc))
        })
        .min()
        .expect("There should be a lowest location");
    println!("lowest location: {lowest_location}");
}

fn part_2(lines: Lines) {
    let lines_vec = lines.collect::<Vec<&str>>();
    let mut parts = lines_vec.split(|line| line.is_empty());

    let seeds_str = parts.next().unwrap()[0];
    let seed_pairs = seeds_str
        .replace("seeds: ", "")
        .split(' ')
        .map(|num_str| num_str.parse::<usize>())
        .filter_map(|num| num.ok())
        .collect::<Vec<usize>>();
    let mut seed_ranges: Vec<Range<usize>> = Vec::new();
    for chunk in seed_pairs.chunks_exact(2) {
        let start = chunk[0];
        let len = chunk[1];
        let chunk_range = start..(start + len);
        seed_ranges.push(chunk_range);
    }
    // println!("seed ranges: {seed_ranges:?}");

    // seed-to-soil map:
    // soil-to-fertilizer map:
    // ...
    let mut range_map_collections: Vec<RangeMapCollection> = Vec::new();
    while let Some(mapping_step_str) = parts.next() {
        let mapping_steps = mapping_step_str.iter().skip(1);
        let range_maps = mapping_steps
            .map(|ms| {
                ms.split(' ')
                    .map(|num_str| num_str.parse::<usize>())
                    .filter_map(|num| num.ok())
                    .collect::<Vec<usize>>()
            })
            .map(|range_map_parts| RangeMap {
                dst_start: range_map_parts[0],
                src_start: range_map_parts[1],
                length: range_map_parts[2],
            });
        let mut range_map_collection = RangeMapCollection::new();
        range_maps.for_each(|rm| range_map_collection.add_range_map(rm));
        range_map_collections.push(range_map_collection);
    }

    // This code runs *really* slow (about 120s in release mode)
    let mut lowest_locations: Vec<usize> = Vec::new();
    for range in seed_ranges {
        println!("processing range {range:?} ({} nums)", range.len());
        let lowest_location = range
            .map(|seed| {
                range_map_collections
                    .iter()
                    .fold(seed, |acc, rmc| rmc.resolve(acc))
            })
            .min()
            .expect("There should be a lowest location");
        println!("lowest location: {lowest_location}");
        lowest_locations.push(lowest_location);
    }
    let lowest_location = lowest_locations
        .iter()
        .min()
        .expect("There should be a lowest location");
    println!("lowest location overall: {lowest_location}");
}
