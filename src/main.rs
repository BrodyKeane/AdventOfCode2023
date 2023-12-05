mod day1;

fn main() {
    let calibration_value = 
        day1::get_calibration_value("input_files/day1_part1_long.txt");
    println!("{}", calibration_value);

}

#[cfg(test)]
mod tests {

    use crate::day1;
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
}

