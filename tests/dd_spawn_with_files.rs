use rdd::Dd;
use std::fs;
use std::path::Path;

/// Test case to verify the functionality of the `Dd::spawn()` method with file input and output.
#[test]
fn dd_spawn_with_files() {
    let input_file = "tests/input.txt";
    let output_file = "tests/output.txt";

    // Check if the input file exists, if not, create it with sample content
    if !Path::new(input_file).exists() {
        fs::write(input_file, b"Hello, dd testing!\n").expect("Failed to create input.txt");
    }

    // If the output file already exists, remove it to ensure a clean test run
    if Path::new(output_file).exists() {
        fs::remove_file(output_file).expect("Failed to remove output.txt");
    }

    // Create a new Dd instance and configure it with the input, output, and other options
    let mut dd = Dd::new("dd");
    dd.input(input_file);
    dd.output(output_file);
    dd.bs("1M");
    dd.count(1);
    dd.status("none");

    let result = dd.spawn();

    assert!(result.is_ok(), "dd command failed");

    // Verify that the output file exists after the command is executed
    assert!(
        Path::new(output_file).exists(),
        "Output file does not exist"
    );

    // Read the content of the output file
    let output_content = fs::read_to_string(output_file).expect("Failed to read output.txt");

    // Check if the output content contains the expected input text
    assert!(
        output_content.contains("Hello, dd testing!"),
        "Output does not contain the expected text"
    );
}
