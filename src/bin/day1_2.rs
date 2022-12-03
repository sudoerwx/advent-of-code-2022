fn main() {
    let data: &str = include_str!("../data/day1.txt");

    let mut numbers_list: Vec<u64> = data
        .trim()
        .split("\n\n")
        .map(|calories_chunk| {
            calories_chunk
                .split("\n")
                .map(|calories| calories.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect::<Vec<u64>>();

    numbers_list.sort_by(|a, b| b.partial_cmp(a).unwrap());
    numbers_list.truncate(3);

    let result: u64 = numbers_list.into_iter().sum();

    println!("{:?}", result);
}
