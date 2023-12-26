pub fn main() {
    let contents = utils::read_input("day2.txt");
    let mut sum: i32 = 0;

    for line in contents.lines() {
        let end = line.find(":").expect("Input is malformed");

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for game in line[end + 2..].split("; ") {
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
                        if number > red {
                            red = number;
                        }
                    }
                    "green" => {
                        if number > green {
                            green = number;
                        }
                    }
                    "blue" => {
                        if number > blue {
                            blue = number;
                        }
                    }
                    _ => panic!("Input is malformed"),
                }
            }
        }

        sum += red * green * blue;
    }

    println!("Solution to Day 2 Part One: {}", sum);
}
