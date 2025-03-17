use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Define the input/output directory
    let test_dir = "tests/inout";
    let out_dir = env::var("OUT_DIR").unwrap();
    let generated_test_file = Path::new(&out_dir).join("generated_tests.rs");

    // Collect all .in and .out file pairs
    let entries = fs::read_dir(test_dir).expect("Failed to read test directory");
    let mut test_cases = vec![];

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("in") {
            let output_file = path.with_extension("out");
            if output_file.exists() {
                test_cases.push((path, output_file));
            } else {
                panic!("No matching .out file for {}", path.display());
            }
        }
    }

    // Load the test function template from the file
    let test_template = fs::read_to_string("test_template.txt")
        .expect("Failed to read test template file");

    // Generate Rust test code
    let mut test_code = String::new();
    test_code.push_str("use super::*;\n\n");

    for (input_file, output_file) in test_cases {
        let test_name = input_file
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .replace("-", "_");

        let input_content = fs::read_to_string(&input_file).expect("Failed to read input file");
        let expected_output = fs::read_to_string(&output_file).expect("Failed to read output file");

        // Format the test template with actual values
        test_code.push_str(&test_template
            .replace("{test_name}", &test_name)
            .replace("{input_content}", &escape_string(&input_content))
            .replace("{expected_output}", &escape_string(&expected_output))
            .replace("{input_file}", &input_file.display().to_string()));
    }

    // Write the generated test code to a file
    fs::write(&generated_test_file, test_code).expect("Failed to write generated test file");
    println!("cargo:rerun-if-changed={}", test_dir);
}

/// Helper function to escape strings for use in Rust raw string literals
fn escape_string(s: &str) -> String {
    s.replace("\"", "\\\"")
}
