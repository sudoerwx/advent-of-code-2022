fn main() {
    let data = include_str!("../data/day3.txt")
        .trim()
        .split("\n")
        .map(str::trim);

    let abc = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let result = data
        .map(|total_items| {
            let (first_compartment, second_compartment) =
                total_items.split_at(total_items.len() / 2);

            let diff_item = first_compartment
                .chars()
                .find(|&item| second_compartment.contains(item))
                .unwrap();

            return abc.find(diff_item).unwrap() + 1;
        })
        .sum::<usize>();

    println!("{:?}", result);
}
