use std::collections::HashMap;

fn main() {
    let file = include_str!("input2.txt");
    let result = process_part1(file);
    println!("{}", result);
}

pub fn process_part1(input: &str) -> String {
    input.lines().map(|line| {
        let game_id = line.split(": ").next().unwrap().split(' ').last().unwrap();
        let cubes_sets = line.split(": ").last().unwrap().split("; ");

        for set in cubes_sets {
            let mut cubes_map = HashMap::from([
                ("red", 12),
                ("green", 13),
                ("blue", 14)
            ]);

            let cubes = set.split(", ").collect::<Vec<&str>>();

            for cube in cubes {
                let count = cube.split(' ').next().unwrap().parse::<i32>().unwrap(); 
                let cube = cube.split(' ').last().unwrap();
                cubes_map.insert(cube, cubes_map.get(cube).unwrap() - count);

                if *cubes_map.get(cube).unwrap() < 0 {
                    return 0;
                }
            }
        }
        
        game_id.parse::<u32>().unwrap()
    }).sum::<u32>().to_string()
}

pub fn process_part2(input: &str) -> String {
    todo!()
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
        assert_eq!("8", process_part1(input));
    }

    #[test]
    fn test_process_day2() {

    }
}
