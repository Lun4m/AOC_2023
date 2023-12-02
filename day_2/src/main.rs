fn part_1(input: &str) -> i32 {
    let mut result = 0;

    for line in input.lines() {
        let split: Vec<_> = line.split([':', ',', ';']).collect();
        let game_id: i32 = split[0][5..].parse().unwrap();
        let mut flag = true;

        for sect in &split[1..] {
            if !match sect.split_whitespace().collect::<Vec<_>>()[..] {
                [n, "red"] => n.parse::<i32>().unwrap() <= 12,
                [n, "green"] => n.parse::<i32>().unwrap() <= 13,
                [n, "blue"] => n.parse::<i32>().unwrap() <= 14,
                _ => true,
            } {
                flag = false;
                break;
            }
        }
        if flag {
            result += game_id;
        }
    }
    result
}

fn part_2(input: &str) -> u32 {
    let mut result: u32 = 0;

    for line in input.lines() {
        let mut min_num = [0, 0, 0];
        let split: Vec<_> = line.split([':', ',', ';']).collect();

        for sect in &split[1..] {
            match sect.split_whitespace().collect::<Vec<_>>()[..] {
                [n, "red"] => {
                    let n = n.parse::<u32>().unwrap();
                    if n > min_num[0] {
                        min_num[0] = n;
                    }
                }
                [n, "green"] => {
                    let n = n.parse::<u32>().unwrap();
                    if n > min_num[1] {
                        min_num[1] = n;
                    }
                }
                [n, "blue"] => {
                    let n = n.parse::<u32>().unwrap();
                    if n > min_num[2] {
                        min_num[2] = n;
                    }
                }
                _ => (),
            }
        }
        result += min_num.iter().product::<u32>();
    }
    result
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
