use std::{fs, u64, ops::Range, mem, cmp::{min, max}};

type Ranges = Vec<Range<u64>>;

pub fn find_nearest_viable_location(path: &str) -> u64 {
    let file = fs::read_to_string(path)
        .expect(&format!("File: {path} could not be read."));

    let headers: [&str; 7] = [
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    let mut segments: [&str; 8] = Default::default();
    let mut segment: &str;
    
    let mut remaining = file.strip_prefix("seeds: ").unwrap();

    for (i, header) in headers.iter().enumerate() {
        (segment, remaining) = remaining.split_once(header).unwrap();
        segments[i] = segment;
    } 

    segments[7] = remaining;

    let seed_ranges: Ranges = 
        segments[0].split(" ")
            .filter_map(|s| s.trim().parse::<u64>().ok())
            .collect::<Vec<u64>>()
            .chunks(2)
            .map(|c| c[0] .. (c[0] + c[1]) )
            .collect::<Ranges>();

    let map_lists: Vec<Vec<Map>> = 
        segments.iter()
            .skip(1)
            .map(|s| parse_maps(s))
            .collect();


    let mut input_ranges: Ranges = seed_ranges;
    let mut mapped_ranges: Ranges = vec![];

    for map_list in map_lists {

        while !input_ranges.is_empty() {
            mapped_ranges.append(&mut map_ranges(&map_list, input_ranges.pop().unwrap()));
        } 

        input_ranges = mem::take(&mut mapped_ranges);
    }

    let mut nearest_location = u64::MAX;

    for range in input_ranges {
        println!("{}", range.start);
        if range.start < nearest_location {
            nearest_location = range.start;
        }
    }

    return nearest_location
}

fn map_ranges(map_list: &Vec<Map>, input_range: Range<u64>) -> Ranges {
    let mut unmapped: Ranges = vec![input_range];
    let mut mapped: Ranges = vec![];

    let mut start: u64;
    let mut end: u64;

    let mut mapped_start;
    let mut mapped_end;

    let mut input: Range<u64>;

    while !unmapped.is_empty() {
        input = unmapped.pop().unwrap();

        for map in map_list {

            if input.start > map.input.end || map.input.start > input.end {
                continue 
            }

            start = max(input.start, map.input.start);
            end = min(input.end, map.input.end);

            mapped_start = map.destination.start + start - map.input.start;
            mapped_end = mapped_start + end - start;

            mapped.push(mapped_start .. mapped_end);
            
            if input.start != start {
                unmapped.push(input.start..start - 1);
            }

            if input.end != end {
                unmapped.push((end + 1) .. (input.end));
            }

            break
        }

        mapped.push(input);
    }

    return mapped
}

fn parse_maps(maps_str: &str) -> Vec<Map> {
    let maps: Vec<Map> = 
        maps_str.split_whitespace()
            .filter_map(|num| num.parse::<u64>().ok())
            .collect::<Vec<u64>>()
            .chunks(3)
            .map(|c| Map::new(c[0], c[1], c[2]))
            .collect();
    return maps
}

pub struct Map {
    destination: Range<u64>,
    input: Range<u64>,
}

impl Map {
    pub fn new(destination: u64, input: u64, length:u64) -> Self {
        Map {
            destination: destination..(destination + length - 1),
            input: input..(input + length - 1)
        }
    }
}
