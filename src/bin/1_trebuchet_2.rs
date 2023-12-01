use regex::Regex;

fn sum(reader: impl std::io::BufRead) -> usize {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let first_regex = Regex::new(&"(?:(0)"
        .chars()
        .chain(words.iter().enumerate().flat_map(|(i, &word)| {
            ['|', '(', ('1' as u8 + i as u8) as char, '|'].into_iter()
            .chain(word.chars())
            .chain(")".chars())
        }))
        .chain(")".chars()).collect::<String>()).unwrap();
    let last_regex = Regex::new(&"(?:(0)"
        .chars()
        .chain(words.iter().enumerate().flat_map(|(i, &word)| {
            ['|', '(', ('1' as u8 + i as u8) as char, '|'].into_iter()
            .chain(word.chars().rev())
            .chain(")".chars())
        }))
        .chain(")".chars()).collect::<String>()).unwrap();
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            let first_digit = first_regex
                .captures(&line)
                .unwrap()
                .iter()
                .skip(1)
                .position(|capture| capture.is_some())
                .unwrap();
            let mut line = line.into_bytes();
            line.reverse();
            let line = String::from_utf8(line).unwrap();
            sum += 10 * first_digit
                + last_regex
                    .captures(&line)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .position(|capture| capture.is_some())
                    .unwrap();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            281,
            sum("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
