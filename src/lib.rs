use ::std::fs::File;
use ::std::io::{BufRead, BufReader};

pub fn blank_lines<F>(sequential_number: usize, filename: F) -> usize
    where F: ::std::convert::AsRef<::std::path::Path> {
    
    let file = match File::open(filename) {
        Ok(o)  => o,
        Err(e) => { panic!(e.to_string()); }
    };
    let mut file = BufReader::new(&file);
    let mut blank_counter: usize = sequential_number;
    let mut final_counter: usize = 0;
    
    loop {
        let mut buf = String::new();
        match file.read_line(&mut buf) {
            Ok(0)  => { break; }, // EOF
            Ok(_)  => { },
            Err(e) => { panic!("{}", e.to_string()); },
        }

        /*for c in buf.chars() {
            print!("{} ", c as u32);
        }
        println!("");*/
        
        if buf.starts_with("\n") || buf.starts_with("\r") {
            blank_counter += 1;
        } else if blank_counter >= sequential_number {
            blank_counter  = 0;
            final_counter += 1;
        }
    }
    
    final_counter
}
