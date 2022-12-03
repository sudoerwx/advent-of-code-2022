fn main() {
    let data = include_str!("../data/day3.txt").trim().split("\n");

    let result: u64 = data
        .map(|calories_chunk| {
            calories_chunk
                .split("\n")
                .map(|calories| calories.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap();

    println!("{:?}", result);
}
