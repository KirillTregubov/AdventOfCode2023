pub fn main() {
    let contents = utils::read_input("day1.txt");

    let mut sum: i32 = 0;

    for line in contents.lines() {
        let new_num: i32 = format!("{}{}", find_number(line, true), find_number(line, false))
            .parse()
            .expect("Input is malformed");
        sum += new_num;
    }

    println!("Solution to Day 1 Part Two: {}", sum);
}

fn find_number(line: &str, finding_first: bool) -> i32 {
    let mut target_index = if finding_first { line.len() } else { 0 };
    let mut target_number = -1;

    let number_words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    for (index, &word) in number_words.iter().enumerate() {
        if finding_first {
            if let Some(position) = line.find(word) {
                if position < target_index {
                    target_index = position;
                    target_number = numbers[index];
                }
            }
        } else {
            if let Some(position) = line.rfind(word) {
                if position > target_index {
                    target_index = position;
                    target_number = numbers[index];
                }
            }
        }
    }

    if (finding_first && target_index == 0) || (!finding_first && target_index == line.len()) {
        return target_number;
    }

    if finding_first {
        let last_num = line
            .find(|c: char| c.is_digit(10))
            .unwrap_or_else(|| line.len());
        if target_number == -1 || last_num < target_index {
            target_number = line.chars().nth(last_num).unwrap().to_digit(10).unwrap() as i32;
        }
    } else {
        let first_num = line.rfind(|c: char| c.is_digit(10)).unwrap_or_else(|| 0);
        if target_number == -1 || first_num > target_index {
            target_number = line.chars().nth(first_num).unwrap().to_digit(10).unwrap() as i32;
        }
    };

    if target_number != -1 {
        return target_number;
    }
    panic!("Input is malformed");
}
