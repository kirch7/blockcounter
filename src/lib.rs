use std::io::{BufRead, BufReader};
use std::iter::Iterator;

pub struct Block<F> {
    buffer:      BufReader<F>,
    last_line:   String,
    tolerable:   usize,
    started:     bool,
}

impl<F> Block<F>
    where F: ::std::io::Read {
    pub fn new(tolerable: usize, stream: F) -> Self {
        Block {
            buffer:      BufReader::new(stream),
            last_line:   String::new(),
            tolerable:   tolerable,
            started:     false,
        }
    }
}

impl<F> Iterator for Block<F>
    where std::io::BufReader<F> : std::io::BufRead {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut block = String::new();
        let mut blank_counter = 0; //self.tolerable;
        loop {
            let mut line = String::new();
            if self.last_line.len() == 0 {
                match &self.buffer.read_line(&mut line) {
                    &Ok(0)  => { break; },
                    &Ok(_)  => {
                        if !self.started {
                            if is_blank(&line) {
                                continue;
                            } else {
                                self.started = true;
                            }
                        }
                    },
                    &Err(ref e) => { panic!("{}", e.to_string()); },
                }
            } else {
                line = self.last_line.clone();
                self.last_line = String::new();
            }
            
            if is_blank(&line) {
                block += &line;
                blank_counter += 1;
            } else {
                if blank_counter >= self.tolerable {
                    if self.tolerable == 0 {
                        block += &line;
                    } else {
                        self.last_line = line.clone();
                    }
                    break;
                } else {
                    block += &line;
                }
                blank_counter = 0;
            }
        }

        if is_blank(&block) {
            None
        } else {
            Some(block)
        }
    }
}

pub fn blank_lines<S>(tolerance: usize, stream: S) -> usize
    where S: ::std::io::Read {
    
    let mut file = BufReader::new(stream);
    let mut blank_counter: usize = tolerance;
    let mut final_counter: usize = 0;
    
    loop {
        let mut line = String::new();
        match file.read_line(&mut line) {
            Ok(0)  => { break; }, // EOF
            Ok(_)  => { },
            Err(e) => { panic!("{}", e.to_string()); },
        }
        
        if is_blank(&line) {
            blank_counter += 1;
        } else {
            if blank_counter >= tolerance {
                final_counter += 1;
            }
            blank_counter = 0;
        }
    }
    
    final_counter
}

fn is_blank(s: &String) -> bool {
    const EMPTY_CHARS: &[char] = &[' ', '\t', '\n', '\r'];

    let mut some_non_blank_char = false;
    for c in s.chars() {
        let mut some_blank_char = false;
        for e in EMPTY_CHARS {
            if c == *e {
                some_blank_char = true;
                break;
            }
        }
        if !some_blank_char {
            some_non_blank_char = true;
            break;
        }
    }
    !some_non_blank_char
}

pub fn clean(s: &String) -> String {
    remove_blank_at_end(&remove_blank_at_beginning(&s))
}

fn remove_blank_at_beginning(s: &String) -> String {
    let mut s = s.clone();
    for line in s.clone().lines() {
        if is_blank(&line.to_string()) {
            if line.len() == 0 {
                let _ = s.remove(0);
            } else {
                for _ in line.chars() {
                    let _ = s.remove(0);
                }
            }
        } else {
            break;
        }
    }
    
    s
}

fn remove_blank_at_end(s: &String) -> String {
    let mut s = s.clone();
    
    loop {
        match s.clone().lines().last() {
            Some(line) => {
                if is_blank(&line.to_string()) {
                    if line.len() == 0 {
                        let _ = s.pop();
                    } else {
                        for _ in line.chars() {
                            let _ = s.pop();
                        }
                    }
                } else {
                    break;
                }
            },
            None       => { break; },
        }
    }
    s
}

#[test]
fn blank_is_blank() {
    let s = " \t\r\n".to_string();
    if !is_blank(&s) {
        panic!("is_blank(&String) problem.");
    }
}

#[test]
fn empty_is_blank() {
    let s = "".to_string();
    if !is_blank(&s) {
        panic!("is_blank(&String) problem.");
    }
}

#[test]
#[should_panic(expected = "is_blank(&String) problem.")]
fn non_blank_is_non_blank() {
    let s = " \t\ra\n".to_string();
    if !is_blank(&s) {
        panic!("is_blank(&String) problem.");
    }
}
