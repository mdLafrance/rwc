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
    
    // Perform requested operation. 
    if args.bytes {
        println!("{:}", get_byte_count(&buffer));
    } else if args.chars {
        println!("{:}", get_char_count(&buffer));
    } else if args.lines{
        println!("{:}", get_line_count(&buffer));
    
    // Special case - no args. Print all.
    } else {
        println!(
            "\t{}\t{}\t{}\t{}",
            get_byte_count(&buffer),
            get_char_count(&buffer),
            get_line_count(&buffer),
            match args.source {
                Some(filename) => filename,
                _ => "".to_string()
            } 
        );
    }

}
