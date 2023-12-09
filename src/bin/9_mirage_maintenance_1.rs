fn sum(reader: impl std::io::BufRead) -> i32 {
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut sequences = vec![line
            .split(" ")
            .map(|value| value.parse::<i32>().unwrap())
            .collect::<Vec<_>>()];
        while sequences.last().unwrap().iter().any(|value| *value != 0) {
            let last = sequences.last().unwrap();
            let mut sequence = Vec::with_capacity(last.len() - 1);
            let mut last = last.iter().copied();
            let mut prev = last.next().unwrap();
            for value in last {
                sequence.push(value - prev);
                prev = value;
            }
            sequences.push(sequence);
        }
        let mut value = 0;
        let mut sequences = sequences.into_iter().rev();
        sequences.next();
        for sequence in sequences {
            value += sequence.last().unwrap();
        }
        sum += value;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            114,
            sum("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"
            .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
