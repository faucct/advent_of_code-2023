use std::collections::HashSet;

fn sum(reader: impl std::io::BufRead) -> u32 {
    let mut sum = 0;
    let mut copies = std::collections::VecDeque::<usize>::new();
    for linecard in reader.lines() {
        let linecard = linecard.unwrap();
        let (winning, yours) = linecard
            .split_once(": ")
            .unwrap()
            .1
            .split_once(" | ")
            .unwrap();

        let winning = winning
            .split(" ")
            .filter(|number| !number.is_empty())
            .map(|number| number.parse().unwrap())
            .collect::<HashSet<u32>>();
        let front = copies.pop_front().unwrap_or(0) + 1;
        let yours = yours
            .split(" ")
            .filter(|number| !number.is_empty())
            .filter(|number| winning.contains(&number.parse().unwrap()))
            .count();
        for i in 0..yours {
            if let Some(copy) = copies.get_mut(i) {
                *copy += front;
            } else {
                copies.push_back(front);
            }
        }
        sum += front;
    }
    sum as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            30,
            sum("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
            .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
