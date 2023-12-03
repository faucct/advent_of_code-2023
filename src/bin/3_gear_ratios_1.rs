fn sum(reader: impl std::io::BufRead) -> u32 {
    let mut sum = 0;
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    for (i, line) in lines.iter().enumerate() {
        let mut adjacent = false;
        let mut number = 0;
        for (j, c) in line.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                number *= 10;
                number += digit;
                for i in i.saturating_sub(1)..(i + 2).min(lines.len()) {
                    let line = &lines[i];
                    for j in j.saturating_sub(1)..(j + 2).min(line.len()) {
                        let c = *line.as_bytes().get(j).unwrap() as char;
                        if !c.is_digit(10) && c != '.' {
                            adjacent = true;
                        }
                    }
                }
            } else {
                if adjacent {
                    sum += number;
                }
                number = 0;
                adjacent = false;
            }
        }
        if adjacent {
            sum += number;
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
            4361,
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
