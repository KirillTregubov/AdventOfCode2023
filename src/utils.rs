use std::fs;

pub fn read_input(file: &str) -> String {
    let path = format!("inputs/{}", file);
    let contents = fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!("Error reading file '{}': {}", path, err);
    });
    return contents;
}
