fn sum(reader: impl std::io::BufRead) -> usize {
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            sum += 10
                * (line
                    .chars()
                    .nth(line.find(char::is_numeric).unwrap())
                    .unwrap() as usize
                    - '0' as usize)
                + (line
                    .chars()
                    .nth(line.rfind(char::is_numeric).unwrap())
                    .unwrap() as usize
                    - '0' as usize);
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
            142,
            sum("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
