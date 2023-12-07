#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Hand {
    High,
    Pair,
    Pairs,
    Three,
    House,
    Four,
    Five,
}

fn sum(reader: impl std::io::BufRead) -> usize {
    let mut hands = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.strip_suffix("\n").unwrap_or(&line);
            fn card(c: char) -> u8 {
                ['T', 'J', 'Q', 'K', 'A']
                    .into_iter()
                    .position(|i| i == c)
                    .map(|position| position + 10)
                    .unwrap_or_else(|| c as usize - '0' as usize) as u8
            }
            fn cards(hand: &str) -> [u8; 5] {
                let mut cards = [0; 5];
                for (i, c) in hand.chars().enumerate() {
                    cards[i] = if c == 'J' { 1 } else { card(c) };
                }
                cards
            }
            fn hand(hand: &str) -> Hand {
                let mut jokers = 0;
                let mut counts = [0u8; 15];
                for c in hand.chars() {
                    if c == 'J' {
                        jokers += 1;
                    } else {
                        counts[card(c) as usize] += 1;
                    }
                }
                for (_, count) in (0..15u8).zip(counts.into_iter()) {
                    if count + jokers == 5 {
                        return Hand::Five;
                    }
                }
                for (_, count) in (0..15u8).zip(counts.into_iter()) {
                    if count + jokers == 4 {
                        return Hand::Four;
                    }
                }
                for (i, count) in counts.into_iter().enumerate() {
                    if count + jokers == 3 {
                        let mut pairs = counts
                            .into_iter()
                            .enumerate()
                            .rev()
                            .filter(|count| count.0 != i && count.1 == 2)
                            .map(|count| count.0 as u8);
                        return if let Some(_) = pairs.next() {
                            Hand::House
                        } else {
                            Hand::Three
                        };
                    }
                }
                for (i, count) in counts.into_iter().enumerate() {
                    if count + jokers == 2 {
                        let mut pairs = counts
                            .into_iter()
                            .enumerate()
                            .rev()
                            .filter(|count| count.0 != i && count.1 == 2)
                            .map(|count| count.0 as u8);
                        return if let Some(_) = pairs.next() {
                            Hand::Pairs
                        } else {
                            Hand::Pair
                        };
                    }
                }
                Hand::High
            }
            let line = line.split_once(" ").unwrap();
            (hand(line.0), cards(line.0), line.1.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();
    hands.sort_unstable();
    let mut sum = 0;
    for (rank, (_, _, bid)) in hands.into_iter().enumerate() {
        sum += (rank + 1) * bid as usize;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            5905,
            sum("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"
            .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
