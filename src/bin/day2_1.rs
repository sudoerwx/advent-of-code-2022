fn main() {
    let data: &str = include_str!("../data/day2.txt");

    fn calc_points(guide: &str) -> u32 {
        match guide.trim() {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!(),
        }
    }

    let result: u32 = data.trim().split("\n").map(calc_points).sum();

    println!("{:?}", result);
}
