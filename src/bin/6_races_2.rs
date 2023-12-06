fn product(reader: impl std::io::BufRead) -> u64 {
    let mut lines = reader.lines();
    let mut time = 0;
    for digit in lines
        .next()
        .unwrap()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .chars()
    {
        if let Some(digit) = digit.to_digit(10) {
            time *= 10;
            time += digit as u64;
        }
    }
    let mut distance = 0;
    for digit in lines
        .next()
        .unwrap()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .chars()
    {
        if let Some(digit) = digit.to_digit(10) {
            distance *= 10;
            distance += digit as u64;
        }
    }
    let mut speed = 0..time / 2 + 1;
    while !speed.is_empty() {
        let pivot = (speed.start + speed.end) / 2;
        if distance < pivot * (time - pivot) {
            speed.end = pivot;
        } else {
            speed.start = pivot + 1;
        }
    }
    time - 2 * speed.end + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            71503,
            product(
                "Time:      7  15   30
Distance:  9  40  200"
                    .as_bytes()
            )
        );
    }
}

fn main() {
    println!("{}", product(std::io::stdin().lock()));
}
