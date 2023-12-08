use std::{fs, usize};

use if_chain::if_chain;

const RADIX: u32 = 10;

pub struct SchematicParser {
    schematic: Vec<Vec<char>>,
    line: usize,
    idx: usize,
}

impl SchematicParser {
    pub fn new(path: &str) -> Self {
        let file = fs::read_to_string(path)
            .expect(&format!("File: {path} could not be read."));

        let lines:Vec<&str> = file.lines().collect();
        
        let mut schematic: Vec<Vec<char>> = vec![];

        for line in lines {
            schematic.push(line.chars().collect());
        }

        return SchematicParser { schematic, line: 0, idx: 0 }
    }

    pub fn sum_of_part_numbers(&mut self) -> u32 {
        let mut sum = 0;

        while !self.is_end_of_file() {
            let next_int = match self.find_next_int() {
                Some(int) => int,
                None => continue,
            };
            if self.is_part_number(&next_int) {
                sum += next_int.value;
            }
        }
        return sum;
    }

    fn find_next_int(&mut self) -> Option<IntInfo> {
        let mut int_info: Option<IntInfo> = None;
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;
        let mut line: Option<usize> = None;
        let mut value: u32 = 0;
        let mut curr: char;

        while start.is_none() {
            curr = match self.current() {
                Some(c) => c,
                None => break,
            };

            if curr.is_numeric() {
                start = Some(self.idx);
                end = Some(self.idx);
                line = Some(self.line);
                value += curr.to_digit(RADIX).unwrap();
            }

            self.next();

        }

        loop {
            curr = match self.current() {
                Some(c) => c,
                None => break,
            };

            if curr.is_numeric() {
                value *= 10;
                value += curr.to_digit(RADIX).unwrap();
                end = Some(self.idx);
            }

            else {
                break
            }

            self.next();

            if self.idx == 0 {
                break
            }
        }


        if_chain! {
            if let Some(start) = start;
            if let Some(end) = end;
            if let Some(line) = line;
            then {
                int_info = Some(IntInfo::new(value, line, start, end));
            }
        }
        return int_info
    }

    fn is_part_number(&self, int_info: &IntInfo) -> bool {
        let mut curr: char;
        let line_range = match int_info.line {
            0 => 0..=1,
            _ => (int_info.line - 1) ..= (int_info.line + 1),
        };

        let idx_range = match int_info.start {
            0 => 0 ..= (int_info.end + 1),
            _ => (int_info.start - 1) ..= (int_info.end + 1),
        };

        for line in line_range {
            if line >= self.schematic.len() {
                continue
            }

            for idx in idx_range.clone() {
                if idx >= self.schematic[line].len() {
                    continue
                }
                
                curr = self.schematic[line][idx];

                if curr.is_numeric() || curr == '.' {
                    continue
                }
                
                return true
            }
        }
        return false
    }

    pub fn calculate_gear_ratios(&mut self) -> u32 {
        let mut sum = 0;

        while !self.is_end_of_file() {
            if let Some(ratio) = self.next_gear_ratio() {
                sum += ratio;
            }
        }
            
        return sum
    }

    fn next_gear_ratio(&mut self) -> Option<u32> {
        self.find_next_gear();

        let line_range = match self.line {
            0 => 0..=1,
            _ => (self.line - 1) ..= (self.line + 1),
        };
        
        let idx_range = match self.idx {
            0 => 0..=1,
            _ => (self.idx - 1) ..= (self.idx + 1),
        };

        let mut part_numbers: Vec<u32> = vec![];
        let mut in_num = false;

        for line in line_range {
            if line >= self.schematic.len() {
                break
            }
            
            for idx in idx_range.clone() {
                if idx > self.schematic[line].len() {
                    break
                }

                if self.schematic[line][idx].is_digit(RADIX) {
                    match in_num {
                        false => {
                            part_numbers.push(self.get_part_number(line, idx));
                            in_num = true;
                        },
                        true => continue,
                    }
                } else {
                    in_num = false;
                }
            }
        }

        let gear_ratio = match part_numbers.len() {
            2 => Some(part_numbers[0] * part_numbers[1]),
            _ => None,
        };

        return gear_ratio
    }

    fn find_next_gear(&mut self) {
        let mut curr: char;
        self.next();
        while !self.is_end_of_file() {
            curr = match self.current() {
                Some(c) => c,
                None => break,
            };

            if curr == '*' {
                break
            }

            self.next();
        }
    }

    fn get_part_number(&self, line: usize, mut idx: usize) -> u32 {
        loop {
            if idx == 0 || !self.schematic[line][idx - 1].is_digit(RADIX) {
                break
            }
            idx -= 1;
        }
        
        let mut part_number: u32 = 0;

        while idx < self.schematic[line].len() {
            if !self.schematic[line][idx].is_digit(RADIX) {
                break
            }

            part_number *= 10;
            part_number += self.schematic[line][idx].to_digit(RADIX).unwrap();
            idx += 1;
        }

        return part_number
    }


    fn next(&mut self) {
        if self.is_end_of_file() {
            return 
        }

        self.idx += 1;
        
        if self.idx >= self.schematic[self.line].len() {
            self.line += 1;
            self.idx = 0;
        }
    }

    fn current(&self) -> Option<char> {
        if self.is_end_of_line() || self.is_end_of_file() {
            return None
        }
        return Some(self.schematic[self.line][self.idx])
    }

    fn is_end_of_file(&self) -> bool {
        return self.is_end_of_line() && self.line >= self.schematic.len() - 1

    }

    fn is_end_of_line(&self) -> bool {
        if !self.line_exists() {
            return true
        }
        return self.idx >= self.schematic[self.line].len();
    }

    fn line_exists(&self) -> bool{
        return self.line < self.schematic.len()
    }
    
}

struct IntInfo {
    value: u32,
    line: usize,
    start: usize,
    end: usize,
}

impl IntInfo {
    pub fn new(value: u32, line: usize, start: usize, end: usize) -> Self {
        IntInfo { value, line, start, end }
    }
}
