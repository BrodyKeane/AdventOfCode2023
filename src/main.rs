#![allow(dead_code, non_snake_case)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let path = "input_files/day7_part1_long.txt";
    let winnings = day7::total_winnings(path);
    println!("{}", winnings);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day3::SchematicParser;

    #[test]
    fn day1_part1_short() {
        let path = "input_files/day1_part1_short.txt";
        let calibration_value = day1::get_calibration_value(path);
        assert_eq!(calibration_value, 142);
    }

    #[test]
    fn day1_part2_short() {
        let path = "input_files/day1_part2_short.txt";
        let calibration_value = day1::get_calibration_value(path);
        assert_eq!(calibration_value, 281);

    }

    #[test]
    fn day1_part2_long() {
        let path = "input_files/day1_part1_long.txt";
        let calibration_value = day1::get_calibration_value(path);
        assert_eq!(calibration_value, 53312);
    }

    #[test]
    fn day2_part1_short() {
        let path = "input_files/day2_part1_short.txt";
        let sum_of_valid_games = day2::verify_games(path);
        assert_eq!(sum_of_valid_games, 8);
    }

    #[test]
    fn day2_part1_long() {
        let path = "input_files/day2_part1_long.txt";
        let sum_of_valid_games = day2::verify_games(path);
        assert_eq!(sum_of_valid_games, 2268);
    }

    #[test]
    fn day2_part2_short() {
        //part 1 files get reused
        let path = "input_files/day2_part1_short.txt";
        let sum_of_powers = day2::sum_of_powers_of_games(path);
        assert_eq!(sum_of_powers, 2286);
    }

    #[test]
    fn day2_part2_long() {
        let path = "input_files/day2_part1_long.txt";
        let sum_of_valid_games = day2::sum_of_powers_of_games(path);
        assert_eq!(sum_of_valid_games, 63542);
    }

    #[test]
    fn day3_part2_short() {
        let path = "input_files/day3_part1_short.txt";
        let mut schematic_parser = SchematicParser::new(path);
        let sum_of_part_numbers = schematic_parser.sum_of_part_numbers();
        assert_eq!(sum_of_part_numbers, 4361);
    }

    #[test]
    fn day3_part2_long() {
        let path = "input_files/day3_part1_long.txt";
        let mut schematic_parser = SchematicParser::new(path);
        let sum_of_part_numbers = schematic_parser.sum_of_part_numbers();
        assert_eq!(sum_of_part_numbers, 514969);
    }

    #[test]
    fn day4_part1_short() {
        let path = "input_files/day4_part1_short.txt";
        let points = day4::find_total_points(path);
        assert_eq!(points , 13);
    }

   // fn day4_part1_long() {
   //     let path = "input_files/day4_part1_long.txt";
   //     let points = day4::find_total_points(path);
   //     assert_eq!(points , );
   // }
}
