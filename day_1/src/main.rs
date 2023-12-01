fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_numeric())
                .map(|c| (c as u8 - 48) as i32)
                .collect::<Vec<i32>>()
        })
        .map(|vec| vec[0] * 10 + vec[vec.len() - 1])
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1 result: {}", part_1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        assert_eq!(part_1(input), 142);
    }
}
