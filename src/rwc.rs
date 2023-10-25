/// Helper functions for rwc cli

use std::{error::Error, fs};

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

    /// Read from the given file instead of from source
    #[arg(short, long)]
    pub file: Option<String>,

    /// String buffer which will be read from
    #[arg(value_name = "SOURCE", index=1)]
    pub source: Option<String>
}


/// Resolves the requested buffer type based on command line input arguments.
/// 
/// rwc can either be used on a file, or from stdin. This function determines the appropriate
/// buffer to read, and returns it.
pub fn get_buffer_from_args(args: &RWCArgs) -> Result<String, Box<dyn Error>> {
    let buffer: String;

    if let Some(file) = &args.file {
        buffer = read_file_contents(file.clone())?;

    } else if let Some(source) = &args.source {
        buffer = source.to_string();

    } else {
        return Err("No source or file path provided".into());
    }

    return Ok(buffer);

}

/// Attempts to read the contents of the given file file_path and return them.
pub fn read_file_contents(file_path: String) -> Result<String, Box<dyn Error>> {
    let file_contents= fs::read_to_string(file_path)?;

    return Ok(file_contents);
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
