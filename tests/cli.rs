#![feature(slice_pattern)]

// Integration test for the whole cli

#[cfg(test)]
mod integration_tests {
    use assert_cmd::prelude::*;
    use std::env;
    use std::path::Path;
    use std::process::Command;

    use rwc::test_utils::get_test_buffer_path;

    fn get_relative_path(target_path: String) -> String {
        let crate_root =
            env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest dir not defined in environment.");

        let result = Path::new(&target_path)
            .strip_prefix(Path::new(&crate_root))
            .expect("Could not strip prefix path.");

        return "./".to_string()
            + result
                .to_str()
                .expect("Couldn't retrieve string from result.");
    }

    /// Returns a command object which already has the relative path of the test buffer bound as an argument.
    fn get_command_for_filepath() -> Command {
        let mut cmd = Command::cargo_bin("rwc").expect("Failed to run cli binary");

        let relative_file_path = get_relative_path(get_test_buffer_path());

        cmd.arg(relative_file_path);

        cmd
    }

    fn decode_stdout(stdout: &[u8]) -> String {
        String::from_utf8(stdout.to_vec()).expect("Could not decode stdout as utf-8")
    }

    #[test]
    /// Test the default output of the rwc command
    fn test_default() {
        // Format test file location to be relative
        let output = get_command_for_filepath()
            .output()
            .expect("Failed to execute command.");

        const EXPECTED_OUTPUT: &str = "43\t177\t1038\t./resources/test/test_input.txt\n";

        assert_eq!(
            decode_stdout(output.stdout.as_slice()),
            EXPECTED_OUTPUT.to_string()
        );
    }

    #[test]
    fn test_get_line_count() {
        let output = get_command_for_filepath()
            .arg("-l")
            .output()
            .expect("Failed to check for line counts.");

        const EXPECTED_OUTPUT: &str = "43\n";

        assert_eq!(
            decode_stdout(output.stdout.as_slice()),
            EXPECTED_OUTPUT.to_string()
        );
    }

    #[test]
    fn test_get_char_count() {
        let output = get_command_for_filepath()
            .arg("-c")
            .output()
            .expect("Failed to check for line counts.");

        const EXPECTED_OUTPUT: &str = "1038\n";

        assert_eq!(
            decode_stdout(output.stdout.as_slice()),
            EXPECTED_OUTPUT.to_string()
        );
    }

    #[test]
    fn test_get_word_count() {
        let output = get_command_for_filepath()
            .arg("-w")
            .output()
            .expect("Failed to check for line counts.");

        const EXPECTED_OUTPUT: &str = "177\n";

        assert_eq!(
            decode_stdout(output.stdout.as_slice()),
            EXPECTED_OUTPUT.to_string()
        );
    }
}
