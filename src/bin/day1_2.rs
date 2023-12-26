use utils::read_input;

fn main() {
    let contents = read_input("day1.txt");

    let mut sum: i32 = 0;

    for line in contents.lines() {
        let first_num = line
            .find(|c: char| c.is_digit(10))
            .expect("Input is malformed");
        let last_num = line
            .rfind(|c: char| c.is_digit(10))
            .expect("Input is malformed");

        let new_num: i32 = format!(
            "{}{}",
            line.chars().nth(first_num).unwrap(),
            line.chars().nth(last_num).unwrap()
        )
        .parse()
        .expect("Input is malformed");
        sum += new_num;
    }

    println!("Solution to Day 1 Part Two: {}", sum);
}
