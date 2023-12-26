pub fn main() {
    // NOTE: If needed, change the max values to the max number of cubes of each color
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let contents = utils::read_input("day2.txt");

    let mut sum: i32 = 0; // mut

    for line in contents.lines() {
        let end = line.find(":").expect("Input is malformed");
        let mut ignore = false;

        'outer: for game in line[end + 2..].split("; ") {
            for cube in game.split(",") {
                let cube = cube.trim();
                let color = cube.split(" ").nth(1).expect("Input is malformed");
                let number = cube
                    .split(" ")
                    .nth(0)
                    .expect("Input is malformed")
                    .parse::<i32>()
                    .expect("Input is malformed");

                match color {
                    "red" => {
                        if number > max_red {
                            ignore = true;
                            break 'outer;
                        }
                    }
                    "green" => {
                        if number > max_green {
                            ignore = true;
                            break 'outer;
                        }
                    }
                    "blue" => {
                        if number > max_blue {
                            ignore = true;
                            break 'outer;
                        }
                    }
                    _ => panic!("Input is malformed"),
                }
            }
        }

        if !ignore {
            sum += line[5..end].parse::<i32>().expect("Input is malformed");
        }
    }

    println!("Solution to Day 2 Part One: {}", sum);
}
