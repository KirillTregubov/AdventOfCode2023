pub fn read_input(file: &str) -> String {
    use std::fs;

    let path = format!("inputs/{}", file);
    let contents = fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!("Error reading file '{}': {}", path, err);
    });
    return contents;
}

/// Measures the runtime of the provided code and prints the elapsed time.
///
/// # Arguments
///
/// * `code_to_profile` - A closure containing the code to be profiled.
///
/// # Examples
///
/// ```
/// // Example usage of the profile function
/// utils::profile(|| {
///     // Your program logic goes here
/// });
/// ```
///
/// This function prints the elapsed time in seconds and milliseconds.
pub fn profile<F>(code_to_profile: F)
where
    F: FnOnce(),
{
    use std::time::Instant;

    // Record the start time
    let start_time = Instant::now();

    // Call the provided closure
    code_to_profile();

    // Record the end time
    let end_time = Instant::now();

    // Calculate the elapsed time
    let elapsed_time = end_time - start_time;

    // Print the elapsed time in seconds and milliseconds
    println!(
        "Elapsed time: {} seconds, {} milliseconds",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );
}
