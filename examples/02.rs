extern crate blockcounter;

use std::fs::File;
use std::io::Write;

mod deletable;
use deletable::{DELETABLE_FILENAME};

const DELETABLE_TEXT: &[u8] = b"block 0\r\nblock 1\r\n\r\nblock 2\r\n\r\n\r\nblock 3\r\n\r\n\r\n\r\nblock 4\r\n\r\n\r\n\r\n\r\nblock 5\r\n\r\n\r\n\r\n\r\n\r\nblock 6\r\n\r\n\r\n\r\n\r\n\r\n\r\nblock 7\r\n\r\n\r\n";

const MAX_EXPECTED_BLOCKS_NO: usize = 8;

fn main() {
    let mut file = File::create(DELETABLE_FILENAME).unwrap();
    let _ = file.write_all(DELETABLE_TEXT);
    let _ = file.flush();

    for tolerance in 0..MAX_EXPECTED_BLOCKS_NO {
        let file = File::open(DELETABLE_FILENAME).unwrap();
        let blocks_no = blockcounter::count_blocks(tolerance, &file);
        assert_eq!(MAX_EXPECTED_BLOCKS_NO - tolerance, blocks_no);
    }

    for tolerance in 0..MAX_EXPECTED_BLOCKS_NO {
        let file = File::open(DELETABLE_FILENAME).unwrap();
        let blocks = blockcounter::Blocks::new(tolerance, &file);
        println!("TOLERANCE: {}", tolerance);
        for block in blocks {
            print!("{}", blockcounter::clean(&block));
            println!("====================================");
        }
        
    }

    
}
