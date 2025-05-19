
#[cfg(test)]
mod tests {
    use treeheart::{HeartRateTree};
    use chrono::{TimeZone, Utc};

    // Tests for Question 2: Did you create the tree correctly and
    // insert the nodes in the right order?
    #[test]
    fn test_insert_nodes() {
        let mut tree = HeartRateTree::new();
        tree.insert(1625150000, 72);
        tree.insert(1625150300, 75);
        tree.insert(1625150600, 78);
        tree.insert(1625140000, 70);
        tree.insert(1625151000, 80);

        let mut nodes = Vec::new();
        tree.inorder_traversal(&tree.root, &mut nodes);

        let expected_nodes = vec![
            (1625140000, 70),
            (1625150000, 72),
            (1625150300, 75),
            (1625150600, 78),
            (1625151000, 80),
        ];

        assert_eq!(nodes, expected_nodes);
    }


     // Tests for Question 3: Is the average calculated correctly?
    #[test]
    fn test_insert_and_average() {
        let mut tree = HeartRateTree::new();
        tree.insert(1625150000, 70);
        tree.insert(1625150300, 80);
        let current_time = 1625150300;
        assert_eq!(tree.average_last_minute(current_time), 80.0);
    }

    #[test]
    fn test_empty_average() {
        let tree = HeartRateTree::new();
        let current_time = 1625150300;
        assert_eq!(tree.average_last_minute(current_time), 0.0);
    }

    #[test]
    fn test_multiple_insertions() {
        let mut tree = HeartRateTree::new();
        tree.insert(1625150000, 70);
        tree.insert(1625150250, 75);
        tree.insert(1625150300, 80);
        let current_time = 1625150300;
        assert_eq!(tree.average_last_minute(current_time), 77.5); // (75+80)/2
    }

    #[test]
    fn test_edge_case_no_data_in_interval() {
        let mut tree = HeartRateTree::new();
        tree.insert(1625150000, 70);
        let current_time = 1625150600; // No data in the last minute
        assert_eq!(tree.average_last_minute(current_time), 0.0);
    }

    #[test]
    fn test_edge_case_single_data_point() {
        let mut tree = HeartRateTree::new();
        tree.insert(1625150540, 70);
        let current_time = 1625150600; // Only one data point in the last minute
        assert_eq!(tree.average_last_minute(current_time), 70.0)
    }

    // Question 4: Does the Display trait work as expected?
    #[test]
    fn test_display_trait() {
        let mut tree = HeartRateTree::new();
        tree.insert(1625150000, 72);
        tree.insert(1625150300, 75);
        tree.insert(1625150600, 78);

        let expected_output = "\
            2021-07-01T14:33:20+00:00 - 72\n\
            2021-07-01T14:38:20+00:00 - 75\n\
            2021-07-01T14:43:20+00:00 - 78\n\
        ";

        let mut output = format!("{}", tree);

        assert_eq!(output, expected_output);
    }
}
