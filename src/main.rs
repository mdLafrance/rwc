use std::process;

use clap::Parser;

mod rwc;
use rwc::*;

fn main() {
    // Parse cli args.
    let args = RWCArgs::parse();

    // Determine appropriate location to read string buffer from.
    let buffer = match get_buffer_from_args(&args) {
        Ok(buffer) => buffer,
        Err(error) => {
            println!("{:}", error);
            process::exit(1);
        }
    };
    
    // Perform requested operation. Defaults to line count.
    let count: usize;

    if args.bytes {
        count = get_byte_count(&buffer);
    } else if args.chars {
        count = get_char_count(&buffer);
    } else {
        count = get_line_count(&buffer);
    }

    println!("{:}", count);
}
