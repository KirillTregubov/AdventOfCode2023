pub fn main() {
    let contents = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
    let mut sum: i32 = 0;

    println!("{}", contents);
    let width = contents.lines().nth(0).unwrap().len();
    let height = contents.lines().count();

    println!("{} {}", width, height);
    sum += 1;

    println!("Solution to Day 3 Part One: {}", sum);
}
