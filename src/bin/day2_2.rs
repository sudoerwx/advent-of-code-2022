fn main() {
    let data: &str = include_str!("../data/day2.txt");

    fn calc_points(guide: &str) -> u32 {
        match guide.trim() {
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            _ => panic!(),
        }
    }

    let result: u32 = data.trim().split("\n").map(calc_points).sum();

    println!("{:?}", result);
}
