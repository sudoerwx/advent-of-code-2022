fn main() {
    let lines = include_str!("../data/day4.txt")
        .trim()
        .lines()
        .map(str::trim);

    let result = lines
        .flat_map(|pair| pair.split(",").flat_map(|range| range.split('-')))
        .flat_map(str::parse)
        .collect::<Vec<usize>>()
        .chunks(4)
        .map(|chunk| {
            chunk[0] <= chunk[2] && chunk[3] <= chunk[1]
                || chunk[2] <= chunk[0] && chunk[1] <= chunk[3]
        })
        .filter(|&bool| bool)
        .count();

    println!("{:?}", result);
}
