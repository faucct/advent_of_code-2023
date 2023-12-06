fn product(reader: impl std::io::BufRead) -> u32 {
    let mut product = 1;
    let mut lines = reader.lines();

    for (time, distance) in lines
        .next()
        .unwrap()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split(" ")
        .filter(|i| !i.is_empty())
        .map(|time| time.parse::<u32>().unwrap())
        .zip(
            lines
                .next()
                .unwrap()
                .unwrap()
                .strip_prefix("Distance:")
                .unwrap()
                .split(" ")
                .filter(|i| !i.is_empty())
                .map(|distance| distance.parse::<u32>().unwrap()),
        )
    {
        let mut speed = 0..time / 2 + 1;
        while !speed.is_empty() {
            let pivot = (speed.start + speed.end) / 2;
            if distance < pivot * (time - pivot) {
                speed.end = pivot;
            } else {
                speed.start = pivot + 1;
            }
        }
        product *= time - 2 * speed.end + 1;
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            288,
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
