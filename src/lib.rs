#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

/// Helper functions for rwc cli

use std::{error::Error, fs, io, };



use clap::Parser;

use mocktopus::macros::*;

#[derive(Parser, Debug)]
#[command(author, about)]
pub struct RWCArgs {
    /// Count number of lines
    #[arg(short, long)]
    pub lines: bool,
    
    /// Count number of characters 
    #[arg(short, long)]
    pub chars: bool,

    /// Count number of whitespace-delineated words
    #[arg(short, long)]
    pub words: bool,

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
        return read_file_contents(source.into());
    
    // Some stdin is detected.
    } else if data_in_stdin() {
        return Ok(read_from_stdin());
        
    // No options provided.
    } else {
        return Err("No source or file path provided".into());
    }
}

/// Counts the number of newline characters in the given buffer.
pub fn get_line_count(buffer: &String) -> usize {
    return buffer.matches('\n').count();
}

/// Get the number of characters in the buffer.
pub fn get_char_count(buffer: &String) -> usize {
    return buffer.chars().count();
}

/// Get count of words in buffer.
pub fn get_word_count(buffer: &String) -> usize {
    return buffer.split_whitespace().count();
}

/// Attempts to read the contents of the given file file_path and return them.
fn read_file_contents(file_path: String) -> Result<String, Box<dyn Error>> {
    let file_contents= fs::read_to_string(file_path)?;

    return Ok(file_contents);
}

/// Check if there is data in stdin.
#[mockable]
fn data_in_stdin() -> bool {
    return !atty::is(atty::Stream::Stdin);
}

/// Collect all existing lines from stdin, and return them as a single string.
#[mockable]
fn read_from_stdin() -> String {
    // Collect all lines into accumulator string.
    // is this fast for large buffers? probably not. Looks rusty though
    let mut acc = io::stdin()
        .lines()
        .filter_map(|line| return line.ok())
        .collect::<Vec<String>>()
        .join("\n");

    // NOTE: Append final newline character to match behavior of wc.
    acc.push('\n');

    return acc;
}

/// Module containing common testing utility for unit and integration tests.
pub mod test_utils {
    use std::env;
    use std::path::Path;

    pub fn get_test_buffer_path() -> String {
        // Get crate root from env
        let crate_root = env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest dir not defined in environment.");

        // Build path to expected test file
        return Path::new(&crate_root)
            .join("resources")
            .join("test")
            .join("test_input.txt")
            .into_os_string()
            .into_string()
            .expect("Could not decode file path into string");
    }
}

#[cfg(test)]
/// Unit tests for the rwc helper lib
pub mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use mocktopus::mocking::*;
    use super::test_utils::*;

    /// Read a known example buffer to use for testing.
    fn get_test_buffer() -> String {
        let buffer_path = get_test_buffer_path();
        let test_buffer_file_path = Path::new(&buffer_path);

        // Check if file can be found
        if !test_buffer_file_path.is_file() {
            panic!("Couldn't find test file: {:?}", test_buffer_file_path);
        }

        // Try and return file contents
        return fs::read_to_string(test_buffer_file_path.as_os_str())
            .unwrap_or_else(|_| panic!("Couldn't read test file: {:?}", test_buffer_file_path));
    }

    #[test]
    /// Test if we can properly load the testing resource
    fn test_get_test_buffer() {
        assert!(!get_test_buffer().is_empty());
    }

    #[test]
    fn test_get_line_count() {
        let buf = get_test_buffer();

        assert_eq!(get_line_count(&buf), 43);
        assert_eq!(get_line_count(&"asdf".to_string()), 0);
    }

    #[test]
    fn test_get_char_count() {
        let buf = get_test_buffer();

        assert_eq!(get_char_count(&buf), 1038);
        assert_eq!(get_char_count(&"asdf".to_string()), 4);
    }

    #[test]
    fn test_get_word_count() {
        let buf = get_test_buffer();

        assert_eq!(get_word_count(&buf), 177);
        assert_eq!(get_word_count(&"asdf".to_string()), 1);
    }

    #[test]
    fn test_read_file_contents() {
        let test_buffer = read_file_contents(get_test_buffer_path()).expect("Couldn't read test buffer");

        assert_eq!(get_test_buffer(), test_buffer);
    }

    #[test]
    fn test_get_buffer_from_args_from_stdin() {
        let dummy_string = "asdf";

        // Mocks out the inner call to read_from_stdin to return a dummy string
        data_in_stdin.mock_safe(|| return MockResult::Return(true));
        read_from_stdin.mock_safe(|| return MockResult::Return(dummy_string.to_string()));

        let args = RWCArgs {
            lines: false,
            chars: false,
            words: false,
            source: None
        };

        assert_eq!(get_buffer_from_args(&args).expect("Failed to run mocked buffer resolution"), dummy_string);
    }

    #[test]
    fn test_get_buffer_from_args_from_file() {
        let args = RWCArgs {
            lines: false,
            chars: false,
            words: false,
            source: Some(get_test_buffer_path())
        };

        assert_eq!(get_buffer_from_args(&args).expect("Failed to run mocked buffer resolution"), get_test_buffer());

    }
}