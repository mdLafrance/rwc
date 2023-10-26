/// Helper functions for rwc cli

use std::{error::Error, fs, io};

use atty;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about)]
pub struct RWCArgs {
    /// Count number of lines
    #[arg(short, long)]
    pub lines: bool,

    /// Count number of bytes
    #[arg(short, long)]
    pub bytes: bool,

    /// Count number of characters 
    #[arg(short, long)]
    pub chars: bool,

    /// String buffer which will be read from
    #[arg(value_name = "SOURCE", index=1)]
    pub source: Option<String>
}


/// Resolves the requested buffer type based on command line input arguments.
/// 
/// rwc can either be used on a file, from the first parameter, or from stdin. 
/// This function determines the appropriate buffer to read, and returns it.
pub fn get_buffer_from_args(args: &RWCArgs) -> Result<String, Box<dyn Error>> {
    // File path supplied as first parameter.
    if let Some(source) = &args.source {
        return Ok(read_file_contents(source.into())?);
    
    // Some stdin is detected.
    } else if data_in_stdin() {
        return Ok(read_from_stdin());
        
    // No options provided.
    } else {
        return Err("No source or file path provided".into());
    }
}

/// Attempts to read the contents of the given file file_path and return them.
pub fn read_file_contents(file_path: String) -> Result<String, Box<dyn Error>> {
    let file_contents= fs::read_to_string(file_path)?;

    return Ok(file_contents);
}

/// Check if there is data in stdin.
pub fn data_in_stdin() -> bool {
    return !atty::is(atty::Stream::Stdin);
}

/// Collect all existing lines from stdin, and return them as a single string.
fn read_from_stdin() -> String {
    // Collect all lines into accumulator string.
    // is this fast for large buffers? probably not. Looks rusty though
    let mut acc = io::stdin()
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>()
        .join("\n");

    // NOTE: Append final newline character to match behavior of wc.
    acc.push_str("\n".into());

    return acc;
}

/// Counts the number of newline characters in the given buffer.
pub fn get_line_count(buffer: &String) -> usize {
    return buffer.matches("\n").count();
}

/// Get the number of characters in the buffer.
pub fn get_char_count(buffer: &String) -> usize {
    return buffer.chars().count();
}

/// Get the number of bytes contained in the buffer
pub fn get_byte_count(buffer: &String) -> usize {
    return buffer.len();
}
