fn main() {
    let file = include_str!("input2.txt");
    let result = process_part2(file);
    println!("{}", result);
}

#[derive(Default)]
struct Turn {
    pub r: usize,
    pub g: usize,
    pub b: usize
}

fn parse_games(input: &str) -> Vec<Vec<Turn>> {
    let mut games = Vec::new();
    for line in input.lines() {
        let mut game = Vec::new();
        let cubes_sets = line.split(": ").last().unwrap().split("; ");

        for set in cubes_sets {
            let cubes = set.split(", ").collect::<Vec<&str>>();

            for cube in cubes {
                let count = cube.split(' ').next().unwrap().parse::<usize>().unwrap(); 
                let cube = cube.split(' ').last().unwrap();

                let mut turn = Turn::default();

                match &cube[..1] {
                    "r" => turn.r = count,
                    "g" => turn.g = count,
                    "b" => turn.b = count,
                    _ => todo!()
                };

                game.push(turn)
            }
        }
        games.push(game);
    }

    games
}

pub fn process_part1(input: &str) -> usize {
    let mut sum = 0;
    'next: for (index, game) in parse_games(input).iter().enumerate() {
        for turn in game {
            if turn.r > 12 || turn.g > 13 || turn.b > 14 {
                continue 'next;
            }
        }
        sum += index + 1;
    } 
    sum
}

pub fn process_part2(input: &str) -> usize {
    let mut sum = 0;
    for game in parse_games(input) {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for turn in game {
            r = r.max(turn.r);
            g = g.max(turn.g);
            b = b.max(turn.b);
        }
        sum += r * g * b;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_day1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process_part1(input));
    }

    #[test]
    fn test_process_day2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, process_part2(input));
    }
}
