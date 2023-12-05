use if_chain::if_chain;

use std::{fs, usize};

pub fn get_calibration_value(path: &str) -> u32 {
    let calibration_file = fs::read_to_string(path)
        .expect(&format!("File: {path} could not be read."));

    let lines = calibration_file.lines();
    let mut line_parser = LineParser::default();

    let mut sum: u32 = 0;
    for line in lines {
        let value = line_parser.parse_line(line);
        if let Some(v) = value {
            sum += v;
        }
    }

    return sum
}

#[derive(Default)]
struct LineParser {
    chars: Vec<char>,
    idx: usize,
    ones_place: Option<u32>,
    tens_place: Option<u32>,
}


impl LineParser {
    pub fn parse_line(&mut self, line: &str) -> Option<u32> {
        self.chars = line.chars().collect();
        self.idx = 0;
        self.ones_place = None;
        self.tens_place = None;

        let first_digit = self.find_next_digit();
        self.tens_place = first_digit;
        self.ones_place = first_digit;
        
        while !self.is_end_of_line() {
            if let Some(digit) = self.find_next_digit() {
                self.ones_place = Some(digit);
            }
        }
        return self.get_value()
    }
    
    fn find_next_digit(&mut self) -> Option<u32> {
        let mut digit: Option<u32> = None;
        let mut char = match self.current() {
            Some(c) => c,
            None => return None,
        };

        while digit.is_none() {
            if let Some(d) = char.to_digit(10) {
                digit = Some(d);
            }

            else if self.next_word_is("one") { digit = Some(1) }
            else if self.next_word_is("two") { digit = Some(2) }
            else if self.next_word_is("three") { digit = Some(3) }
            else if self.next_word_is("four") { digit = Some(4) }
            else if self.next_word_is("five") { digit = Some(5) }
            else if self.next_word_is("six") { digit = Some(6) }
            else if self.next_word_is("seven") { digit = Some(7) }
            else if self.next_word_is("eight") { digit = Some(8) }
            else if self.next_word_is("nine") { digit = Some(9) }

            char = match self.next() {
                Some(c) => c,
                None => break,
            };
        }
        return digit
    }

    fn next_word_is(&self, word: &str) -> bool {
        if self.idx + word.len() > self.chars.len() {
            return false
        }

        for (word_idx, char) in word.chars().enumerate() {
            if char != self.chars[self.idx + word_idx] {
                return false
            }
        }
        return true
    }

    fn get_value(&self) -> Option<u32> {
        if_chain! {
            if let Some(tens_place) = self.tens_place;
            if let Some(ones_place) = self.ones_place;
            then {
                return Some(tens_place * 10 + ones_place)
            }
        }
        return None
    }

    fn current(&self) -> Option<char> {
        if self.idx >= self.chars.len() {
            return None
        }
        return Some(self.chars[self.idx])
    }


    fn next(&mut self) -> Option<char> {
        self.idx += 1;
        if self.is_end_of_line() {
            return None
        }
        return Some(self.chars[self.idx])
    }

    fn is_end_of_line(&self) -> bool {
        return self.idx == self.chars.len()
    }
}
