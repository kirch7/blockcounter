/*!
A crate to count blocks in plain text.

Consider a block a set of lines separated by a given number of empty lines.

# Example

```rust
extern crate blockcounter;
use blockcounter::{count_blocks, Blocks, clean};

fn main() {
    let text = "0\n1\n\n2\n\n\n3\n\n".to_string();
    println!("{}", text);
    println!("===========");
    println!("text has {} blocks.", count_blocks(2, text.as_bytes()));
    println!("======================");
    println!("");
    
    for block in Blocks::new(2, text.as_bytes()) {
        print!("{}", clean(&block));
        println!("=============");
    }
}
```

*/
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

pub struct Blocks<F> {
    buffer:           BufReader<F>,
    last_line:        String,
    tolerable:        usize,
    started:          bool,
    comments:         Vec<String>,
}

#[deprecated(since="0.2.1", note="please, use `Blocks` instead.")]
pub type Block<F> = Blocks<F>;

impl<F> Blocks<F>
    where F: ::std::io::Read {
    pub fn new(tolerable: usize, stream: F) -> Self {
        Blocks {
            buffer:           BufReader::new(stream),
            last_line:        String::new(),
            tolerable:        tolerable,
            started:          false,
            comments:         Vec::new(),
        }
    }

    pub fn new_with_comments(tolerable: usize, stream: F, comments: &Vec<String>) -> Self {
        Blocks {
            buffer:           BufReader::new(stream),
            last_line:        String::new(),
            tolerable:        tolerable,
            started:          false,
            comments:         comments.clone(),
        }
    }
}

impl<F> Iterator for Blocks<F>
    where std::io::BufReader<F> : std::io::BufRead {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut block = String::new();
        let mut blank_counter = 0; //self.tolerable;
        let comment_as_blank = self.comments.len() != 0;
        loop {
            let mut line = String::new();
            if self.last_line.len() == 0 {
                match &self.buffer.read_line(&mut line) {
                    &Ok(0)  => { break; },
                    &Ok(_)  => {
                        if !self.started {
                            if is_blank(&line) || ( /*comment_as_blank && */is_comment(&line, &self.comments) ) {
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
            
            if is_blank(&line) || (/* comment_as_blank && */is_comment(&line, &self.comments) ) {
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
        
        let block_is_garbage = match comment_as_blank {
            false => is_blank(&block),
            true  => {
                let mut all_garbage = true;
                for line in block.lines() {
                    let line = line.to_string();
                    if !is_comment(&line, &self.comments) && !is_blank(&line) {
                        all_garbage = false;
                        break;
                    }
                }
                all_garbage
            },
        };

        if block_is_garbage {
            None
        } else {
            Some(block)
        }
    }
}

#[deprecated(since="0.2.1", note="please, use `count_blocks` instead.")]
pub fn blank_lines<S: ::std::io::Read>(tolerance: usize, stream: S) -> usize {
    count_blocks(tolerance, stream)
}

/// Given a <em>tolerance</em> input, returns the number of blocks is a stream.
pub fn count_blocks<S>(tolerance: usize, stream: S) -> usize
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

fn is_comment(s: &String, comments: &Vec<String>) -> bool {
    if comments.len() == 0 {
        return false;
    }
    
    let mut s = s.clone();

    loop {
        let c0 = s.chars().nth(0);
        if c0.is_none() {
            break;
        }
        let c0 = c0
            .unwrap()
            .to_string();
        if is_blank(&c0) {
            let _ = s.remove(0);
        } else {
            break;
        }
        
    }
    for comment in comments {
        if s.starts_with(comment) {
            return true;
        }
    }
    
    false
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

/// Removes blank lines at the beginnig and at the end of a <em>String</em>.
pub fn clean(s: &String) -> String {
    remove_blank_at_end(&remove_blank_at_beginning(&s))
}

/// Removes all blank lines of a <em>String</em>.
pub fn clean_all_blank(s: &String) -> String {
    let mut t = String::new();
    for line in s.lines() {
        if !is_blank(&line.to_string()) {
            t += &line;
        }
    }
    t
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

#[test]
fn comment_is_comment() {
    let s = "\t//fldfjbas".to_string();
    if !is_comment(&s, &vec!["//".to_string()]) {
        panic!("is_blank(&String) problem.");
    }
}

#[test]
#[should_panic(expected = "is_comment(&String, &Vec<_>) problem.")]
fn non_comment_is_non_comment() {
    let s = " bhfjass".to_string();
    if !is_comment(&s, &vec!["//".to_string()]) {
        panic!("is_comment(&String, &Vec<_>) problem.");
    }
}

#[test]
fn blank_is_not_comment() {
    let s = " \t\r\n".to_string();
    if is_comment(&s, &vec!["a".to_string()]) {
        panic!("is_comment(...) problem.");
    }
}

#[test]
fn clean_test() {
    let s = " \t\r\n\n\n\t\r \n".to_string();
    assert_eq!(clean(&s), "".to_string());
    let s = "a\t\r\n\n\n\t\r \n".to_string();
    assert_eq!(clean(&s), "a\t\r\n".to_string());
}

#[test]
fn clean_all_test() {
    let s = " \t\r\n\n\n\n\t\r  \n".to_string();
    assert_eq!(clean_all(&s), "".to_string());
}

