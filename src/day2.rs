use std::fs;

const RADIX: u32 = 10;
const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Default)]
struct GameParser {
    game: Vec<char>,
    idx: usize,
}

struct CubeCount {
    color: CubeColor,
    count: u32,
}

struct GameInfo {
    id: u32,
    min_red: u32,
    min_blue: u32,
    min_green: u32,
}

pub fn verify_games(path: &str) -> u32 {
    let games = parse_games(path);
    let mut sum = 0;

    for game in games {
        if game.is_valid() { sum += game.id }
    }
    return sum
}

pub fn sum_of_powers_of_games(path: &str) -> u32 {
    let games = parse_games(path);
    let mut sum = 0;

    for game in games {
        sum += game.get_power();
    }
    return sum
}

fn parse_games(path: &str) -> Vec<GameInfo> {
    let file = fs::read_to_string(path)
        .expect(&format!("File: {path} could not be read."));

    let games = file.lines();
    let mut game_parser = GameParser::default();
    let mut parsed_games: Vec<GameInfo> = vec![];

    for game in games {
        match game_parser.parse(game) {
            Some(game_info) => parsed_games.push(game_info),
            None => continue,
        };
    }
    return parsed_games
}

impl GameParser {
    pub fn parse(&mut self, game: &str) -> Option<GameInfo> {
        self.idx = 0;
        self.game = game.chars().collect();
         
        let id = match self.find_next_int() {
            Some(id) => id,
            None => {
                eprintln!("Id could not be found for \n{game}\n");
                return None;
            },
        };

        let mut cube_counts: Vec<CubeCount> = vec!();
        while !self.is_end_of_line() {
            match self.next_cube_count() {
                Some(cube_count) => cube_counts.push(cube_count),
                None => break
            }
        } 

        return Some(GameInfo::new(id, cube_counts));
    }

    fn next_cube_count(&mut self) -> Option<CubeCount> {
        let count: Option<u32> = self.find_next_int();
        let color: Option<CubeColor> = self.find_next_color();

        let cube_count = match (color, count) {
            (Some(color), Some(count)) => Some(CubeCount {color, count}),
            _ => None,
        };

        return cube_count
    }

    fn find_next_color(&mut self) -> Option<CubeColor> {
        let mut color: Option<CubeColor> = None; 
        let mut curr: char;
        while color.is_none() {
            curr = match self.current() {
                Some(c) => c,
                None => break,
            };

            if !curr.is_alphabetic() {
                self.next();
            }

            else if self.next_word_is("red") { color = Some(CubeColor::Red); }
            else if self.next_word_is("green") { color = Some(CubeColor::Green); }
            else if self.next_word_is("blue") { color = Some(CubeColor::Blue); }
        }
        return color
    }

    fn find_next_int(&mut self) -> Option<u32> {
        let mut count: Option<u32> = None;
        let mut curr: char;
        while count.is_none() {
             curr = match self.current() {
                Some(c) => c,
                None => break,
            };

            if !curr.is_numeric() {
                self.next();
            }
            else {
                count = self.parse_int();
            }
        }
        return count
    }

    fn parse_int(&mut self) -> Option<u32> {
        let mut curr: char;
        let mut int = 0;
        while !self.is_end_of_line() {
            curr = match self.current() {
                Some(c) => c,
                None => break,
            };

            let next_digit = match curr.to_digit(RADIX){
                Some(d) => d,
                None => break,
            };
            
            int *= 10;
            int += next_digit;

            self.next();
        }

        return match int {
            0 => None,
            _ => Some(int),
        }
    }

    fn next_word_is(&self, word: &str) -> bool {
        if self.idx + word.len() > self.game.len() {
            return false
        }

        for (word_idx, char) in word.chars().enumerate() {
            if char != self.game[self.idx + word_idx] {
                return false
            }
        }
        return true
    }

    fn current(&self) -> Option<char> {
        if self.is_end_of_line() {
            return None
        }
        return Some(self.game[self.idx])
    }


    fn next(&mut self) -> Option<char> {
        self.idx += 1;
        if self.is_end_of_line() {
            return None
        }
        return Some(self.game[self.idx])
    }

    fn is_end_of_line(&self) -> bool {
        return self.idx >= self.game.len()
    }
}

impl GameInfo {
    pub fn new(id: u32, cube_counts: Vec<CubeCount>) -> Self {
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for cube_count in cube_counts {
            match cube_count.color {
                CubeColor::Red => {
                    if cube_count.count > min_red {
                        min_red = cube_count.count;
                    }
                },
                CubeColor::Green => {
                    if cube_count.count > min_green {
                        min_green = cube_count.count;
                    }
                }
                CubeColor::Blue => {
                    if cube_count.count > min_blue {
                        min_blue = cube_count.count;
                    }
                }
            }
        }

        return GameInfo { id, min_red, min_green, min_blue }
    }

    pub fn get_power(&self) -> u32 {
        return self.min_red * self.min_blue * self.min_green
    }

    fn is_valid(&self) -> bool {
        return {
            self.min_red <= MAX_RED
            && self.min_green <= MAX_GREEN
            && self.min_blue <= MAX_BLUE
        }
    }
}
