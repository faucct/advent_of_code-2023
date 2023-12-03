fn find_number(line: &String, mut j: usize) -> u32 {
    if let Some(c) = line.chars().nth(j) {
        if c.is_digit(10) {
            while j > 0 && line.chars().nth(j - 1).unwrap().is_digit(10) {
                j -= 1;
            }
            let mut number = 0;
            while let Some(c) = line.chars().nth(j) {
                if let Some(digit) = c.to_digit(10) {
                    number *= 10;
                    number += digit;
                    j += 1;
                } else {
                    break;
                }
            }
            return number;
        }
    }
    0
}

fn sum(reader: impl std::io::BufRead) -> u32 {
    let mut sum = 0;
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '*' {
                continue;
            }
            let mut count = 0;
            let mut product = 1;
            for i in i.saturating_sub(1)..(i + 2).min(lines.len()) {
                let line = &lines[i];
                if line.chars().nth(j).unwrap().is_digit(10) {
                    let number = find_number(line, j);
                    if number != 0 {
                        count += 1;
                        product *= number;
                    }
                } else {
                    for j in j.saturating_sub(1)..(j + 2).min(line.len()) {
                        let number = find_number(line, j);
                        if number != 0 {
                            count += 1;
                            product *= number;
                        }
                    }
                }
            }
            if count == 2 {
                sum += product;
            }
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
            467835,
            sum("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
