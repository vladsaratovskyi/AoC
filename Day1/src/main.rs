fn main() {
    let file = include_str!("input2.txt");
    let result = process(file);
    println!("{}", result);
}

pub fn process(input: &str) -> String {
    let lines = input.lines();
    let res = 
        lines.map(|line|
        {
            let digits: String = line.chars().filter(|c| c.is_numeric()).collect();

            let first = digits.chars().next().unwrap().to_digit(10).unwrap();
            let last = digits.chars().last().unwrap().to_digit(10).unwrap();

            first * 10 + last
        }
    ).fold(0, |acc, i| acc + i);

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input));
    }
}
