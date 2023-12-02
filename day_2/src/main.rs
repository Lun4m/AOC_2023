use std::collections::HashMap;

fn part_1(input: &str) -> i32 {
    let cmap = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    input
        .lines()
        .filter_map(|line| {
            let split = line.split([':', ',', ';']).collect::<Vec<_>>();
            let game_id: i32 = split[0][5..].parse().unwrap();

            split[1..]
                .iter()
                .all(|s| {
                    if let [n, color] = s.split_whitespace().collect::<Vec<_>>()[..] {
                        return n.parse::<i32>().unwrap() <= *cmap.get(color).unwrap();
                    }
                    true
                })
                .then_some(game_id)
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut cmap = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            let split: Vec<_> = line.split([':', ',', ';']).collect();
            split[1..].iter().for_each(|s| {
                if let [n, color] = s.split_whitespace().collect::<Vec<_>>()[..] {
                    let n = n.parse::<u32>().unwrap();
                    if n > *cmap.get(color).unwrap() {
                        cmap.insert(color, n);
                    }
                }
            });

            cmap.values().product::<u32>()
        })
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1 result: {}", part_1(input));
    println!("Part 2 result: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!(part_1(input), 8);
    }

    #[test]
    fn test_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ";
        assert_eq!(part_2(input), 2286);
    }
}
