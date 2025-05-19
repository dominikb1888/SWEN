use biomarker::{load_biomarker_data, sum_extrapolated_values};

fn main() -> std::io::Result<()> {
    // Load biomarker data from input.txt
    let input_data = load_biomarker_data("input.txt")?;

    // Compute the sum of extrapolated values
    let result = sum_extrapolated_values(input_data);
    println!("Sum of predicted values: {}", result);

    Ok(())
}

