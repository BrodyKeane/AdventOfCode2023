use std::{fs, collections::HashMap, usize};


pub fn find_total_points(path: &str) -> usize {
    let file = fs::read_to_string(path)
        .expect(&format!("File: {path} could not be read."));

    let lines = file.lines();
    let mut points: usize = 0;

    for line in lines {
        points += find_points(line);
    }

    return points
}

pub fn find_total_cards(path: &str) -> usize {
    let file = fs::read_to_string(path)
        .expect(&format!("File: {path} could not be read."));

    let lines: Vec<&str> = file.lines().collect();

    let mut card_counts: Vec<usize> = vec![1; lines.len()];

    let mut total_cards: usize = 0;
    let mut card_points: usize;

    for (i, line) in lines.iter().enumerate() {
        card_points = find_points(line);
        total_cards += card_counts[i];

        println!("Card {}: Count: {} Points: {}", i + 1, card_counts[i], card_points);
        for j in (i + 1) ..= (i + card_points) {
            if j >= lines.len() {
                break
            }

            card_counts[j] += card_counts[i];
        }
    }

    return total_cards
}

fn find_points(mut line: &str) -> usize {
    line = line.split(":").collect::<Vec<&str>>()[1];
    let number_sets = line.split("|").collect::<Vec<&str>>();

    let winning_number_strings = number_sets[0].split(" ").collect::<Vec<&str>>();
    let mut winning_numbers: Vec<u8> = vec![];

    for mut num_str in winning_number_strings {
        num_str = num_str.trim();

        let num = num_str.parse::<u8>();

        if let Ok(num) = num {
            winning_numbers.push(num);
        }
    }

    let player_number_strings = number_sets[1].split(" ").collect::<Vec<&str>>();
    let mut points: usize = 0;

    for mut num_str in player_number_strings {
        num_str = num_str.trim();

        let num = num_str.trim().parse::<u8>();

        if let Ok(num) = num {
            if winning_numbers.contains(&num) {
                points += 1
            }
        }
    }
    return points
}


