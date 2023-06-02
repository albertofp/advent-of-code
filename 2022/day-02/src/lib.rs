pub fn process_part1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|load| {
            load.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|load| {
            load.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a)); // Reverse sorting order so first 3 are largest

    let sum: u32 = result.iter().take(3).sum();

    sum.to_string()
}
