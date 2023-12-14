use std::{fs, usize,};

pub fn find_wins(path: &str) -> usize {
    let file = fs::read_to_string(path)
        .expect(&format!("File: {path} could not be read."));

    let mut lines = file.lines();

    let times = get_times(lines.next().unwrap());
    let distances = get_distances(lines.next().unwrap());

    return win_product(times, distances)
}

fn get_times(times_str: &str) -> usize {
    times_str.strip_prefix("Time:")
        .expect(&format!("Failed to strip 'Time:' from start of string:\n{}", times_str))
        .split_ascii_whitespace()
        .fold(String::new(), |a, b| a + b)
        .parse::<usize>()
        .expect(&format!("Failed to parse Time from:\n{}", times_str))
}

fn get_distances(distance_str: &str) -> usize {
    distance_str.strip_prefix("Distance:")
        .expect(&format!("Failed to strip 'Distance:' from start of string:\n{}", distance_str))
        .split_ascii_whitespace()
        .fold(String::new(), |a, b| a + b)
        .parse::<usize>()
        .expect(&format!("Failed to parse Distance from:\n{}", distance_str))
}


fn win_product(time: usize, distance: usize) -> usize {
    let mut wins = 0;
    for hold_time in 0..time {
        if distance_traveled(hold_time, time) > distance {
            wins += 1;
        }
    }
    return wins
}

fn distance_traveled(hold_time: usize, total_time: usize) -> usize {
    (total_time - hold_time) * hold_time
}
