use biomarker::{Biomarker, load_biomarker_data, sum_extrapolated_values};

#[cfg(test)]
mod biomarker_tests {
    use super::*;
    use std::fs::File;
    use std::io::{self, Write};

    // Helper function to create a test input file
    fn create_test_file(contents: &str) -> io::Result<String> {
        let file_name = "test_input.txt";
        let mut file = File::create(file_name)?;
        file.write_all(contents.as_bytes())?;
        Ok(file_name.to_string())
    }

    // Test 1: Test that data is loaded correctly from a file
    #[test]
    fn test_load_data_from_file() {
        let file_contents = "1 2 3\n4 5 6\n7 8 9";
        let file_name = create_test_file(file_contents).expect("Failed to create test file");

        let biomarkers = load_biomarker_data(&file_name).expect("Failed to load biomarkers from file");

        assert_eq!(biomarkers.len(), 3);  // Expect 3 biomarker sequences
        assert_eq!(biomarkers[0].sequence, vec![1, 2, 3]);
        assert_eq!(biomarkers[1].sequence, vec![4, 5, 6]);
        assert_eq!(biomarkers[2].sequence, vec![7, 8, 9]);
    }

    // Test 2: Test that Biomarker struct is created correctly
    #[test]
    fn test_biomarker_creation() {
        let biomarker = Biomarker::new(vec![1, 2, 3]);
        assert_eq!(biomarker.sequence, vec![1, 2, 3]);
    }

    // Test 3: Test that the `next_value` method returns the correct type (i64)
    #[test]
    fn test_next_value_type() {
        let biomarker = Biomarker::new(vec![1, 2, 3, 4]);
        let next_value = biomarker.next_value();
        assert_eq!(next_value, 5);  // We expect the next value to be 5 based on the logic
    }

    // Test 4: Test that `next_value` method provides the correct result for simple sequence
    #[test]
    fn test_next_value_simple_sequence() {
        let biomarker = Biomarker::new(vec![1, 3, 5, 7]);
        let next_value = biomarker.next_value();
        assert_eq!(next_value, 9);  // Next value should be 9 in an arithmetic progression (difference of 2)
    }

    // Test 5: Test that `next_value` method handles more complex sequence
    #[test]
    fn test_next_value_complex_sequence() {
        let biomarker = Biomarker::new(vec![1, 4, 10, 20]);
        let next_value = biomarker.next_value();
        assert_eq!(next_value, 35);  // Based on the differences 3, 6, 10, the next value should be 35
    }

    // Test 6: Test the sum of all extrapolated values is correct
    #[test]
    fn test_sum_extrapolated_values() {
        let biomarkers = vec![
            Biomarker::new(vec![1, 2, 3]),  // Extrapolated value should be 4
            Biomarker::new(vec![4, 6, 8]),  // Extrapolated value should be 10
            Biomarker::new(vec![10, 20, 30]) // Extrapolated value should be 40
        ];

        let total_sum = sum_extrapolated_values(biomarkers);
        assert_eq!(total_sum, 54);  // 4 + 10 + 40 = 54
    }

    // Test 7: Test the next value of a sequence with varying differences
    #[test]
    fn test_next_value_varying_differences() {
        let biomarker = Biomarker::new(vec![10, 13, 16, 21, 30]);
        let next_value = biomarker.next_value();
        assert_eq!(next_value, 45);  // Expected value after differences stabilize
    }

    // Test 8: Test next value for a sequence with only one element
    #[test]
    fn test_next_value_with_single_element() {
        let biomarker = Biomarker::new(vec![20]);
        let next_value = biomarker.next_value();
        assert_eq!(next_value, 20);  // With one element, the next value is the same
    }

    // Test 9: Test the behavior with a sequence that has all zeroes
    #[test]
    fn test_sequence_with_all_zeroes() {
        let biomarker = Biomarker::new(vec![0, 0, 0, 0]);
        let next_value = biomarker.next_value();
        assert_eq!(next_value, 0);  // With all zeroes, the next value should be zero
    }

    // Test 10: Test that the `next_value` method works with complex, non-uniform differences
    #[test]
    fn test_non_uniform_sequence() {
        let biomarker = Biomarker::new(vec![1, 3, 8, 15, 24]);
        let next_value = biomarker.next_value();
        assert_eq!(next_value, 36);  // Correct extrapolated value based on differences
    }
}

