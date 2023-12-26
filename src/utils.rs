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

    let start_time = Instant::now();

    code_to_profile();

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    let seconds = elapsed_time.as_secs();
    let milliseconds = elapsed_time.subsec_millis();

    if seconds > 0 {
        println!(
            "Elapsed time: {} seconds, {} milliseconds",
            seconds, milliseconds
        );
        return;
    }
    println!(
        "Elapsed time: {} milliseconds {} microseconds",
        milliseconds,
        elapsed_time.subsec_micros() - milliseconds * 1000
    );
}
