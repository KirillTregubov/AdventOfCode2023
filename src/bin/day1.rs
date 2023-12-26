use utils::read_input;

fn main() {
    println!("Hello, day 1!");

    let file: &str = "day1.txt";
    let contents = read_input(file);

    println!("Read:\n{}", contents);
}
