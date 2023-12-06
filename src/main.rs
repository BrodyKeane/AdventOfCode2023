use day2::verify_games;

mod day1;
mod day2;

fn main() {
    let path = "input_files/day2_part1_long.txt";
    let sum_of_valid_games = verify_games(path);
    println!("{}", sum_of_valid_games);
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn day2_part2_long() {
        let path = "input_files/day2_part1_long.txt";
        let sum_of_valid_games = day2::verify_games(path);
        assert_eq!(sum_of_valid_games, 2268);
    }
}

