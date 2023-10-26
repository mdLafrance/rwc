use std::process;

use clap::Parser;

mod rwc;
use rwc::*;

fn main() {
    // Parse cli args.
    let args = RWCArgs::parse();

    // Determine appropriate location to read string buffer from (stdin or file).
    let buffer = match get_buffer_from_args(&args) {
        Ok(buffer) => buffer,
        Err(error) => {
            println!("{:}", error);
            process::exit(1);
        }
    };
    
    // Perform requested operation. 
    if args.chars {
        println!("{:}", get_char_count(&buffer));
    } else if args.lines{
        println!("{:}", get_line_count(&buffer));
    } else if args.words {
        println!("{:}", get_word_count(&buffer))
    
    } else {
        // Special case - no args. Print all.
        println!(
            "{}\t{}\t{}\t{}",
            get_line_count(&buffer),
            get_word_count(&buffer),
            get_char_count(&buffer),
            args.source.unwrap_or("".to_string())
        );
    }
}
