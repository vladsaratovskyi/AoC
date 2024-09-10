
fn main() {
    let file = include_str!("input2.txt");
    let result = process_part2(file);
    println!("{}", result);
}

pub fn process_part1(input: &str) -> String {
    let lines = input.lines();
    let res = 
        lines.map(|line|
        {
            let digits: String = line.chars().filter(|c| c.is_numeric()).collect();

            let first = digits.chars().next().unwrap().to_digit(10).unwrap();
            let last = digits.chars().last().unwrap().to_digit(10).unwrap();

            first * 10 + last
        }
    ).sum::<u32>();

    res.to_string()
}

pub fn convert_to_num(num: &str) -> i32 {
    match num {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => 0
    }
}

pub fn process_part2(input: &str) -> String {
    let nums = ["nine", "0", "zero", "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven", "8", "eight", "9", ];

    let lines = input.lines();
    let res: i32 = lines.map(|line| {
            let mut l = 0;
            let mut r = line.len();

            let mut left = 0;
            let mut right = 0;

            let mut left_found = false;
            let mut right_found = false;

            while l < line.len() && !left_found {
                for num in nums {
                    let left_slice = &line[l..];
                    if left_slice.starts_with(num) {
                        left = convert_to_num(num);
                        left_found = true;
                        break;
                    }
                }
                l += 1;
            }

            while r >= 0 && !right_found {
                for num in nums {
                    let right_slice = &line[..r];
                    if right_slice.ends_with(num) {
                        right = convert_to_num(num);
                        right_found = true;
                        break;
                    }
                }
                r -= 1;
            }

        left * 10 + right
    }).sum();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_day1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process_part1(input));
    }

    #[test]
    fn test_process_day2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process_part2(input));
    }
}
