fn main() {
    let data: &str = include_str!("../data/day1.txt");

    let result: u64 = data
        .trim()
        .split("\n\n")
        .map(|calories_chunk| {
            calories_chunk
                .split("\n")
                .map(|calories| calories.parse::<u64>().unwrap())
                .sum()
        })
        .max()
        .unwrap();

    println!("{:?}", result);
}
