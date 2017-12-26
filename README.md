# blockcounter
[![Version info](https://img.shields.io/crates/v/blockcounter.svg)](https://crates.io/crates/blockcounter)
[![Build Status](https://travis-ci.org/kirch7/blockcounter.svg?branch=master)](https://travis-ci.org/kirch7/blockcounter)
Count blocks in a text.

### Example

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
