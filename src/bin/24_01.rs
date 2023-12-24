use std::ops::RangeInclusive;

fn sum(test_area: RangeInclusive<u64>, reader: impl std::io::BufRead) -> usize {
    let test_area = *test_area.start() as f64..=*test_area.end() as f64;
    let hailstones = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut line = line.split(" @ ").map(|line| {
                let mut line = line
                    .split(", ")
                    .map(|number| number.parse::<i64>().unwrap());
                (
                    line.next().unwrap(),
                    line.next().unwrap(),
                    line.next().unwrap(),
                )
            });
            (line.next().unwrap(), line.next().unwrap())
        })
        .collect::<Vec<_>>();
    hailstones
        .iter()
        .enumerate()
        .map(|(i, hailstone_a)| {
            hailstones[..i]
                .iter()
                .filter(|hailstone_b| {
                    // hailstone_a.0.0 + hailstone_a.1.0 * t1 == hailstone_b.0.0 + hailstone_b.1.0 * t2;
                    // t1 == (hailstone_b.0.0 - hailstone_a.0.0 + hailstone_b.1.0 * t2) / hailstone_a.1.0;
                    // hailstone_a.0.1 + hailstone_a.1.1 * t1 == hailstone_b.0.1 + hailstone_b.1.1 * t2;
                    // hailstone_a.0.1 + hailstone_a.1.1 * (hailstone_b.0.0 - hailstone_a.0.0 + hailstone_b.1.0 * t2) / hailstone_a.1.0 == hailstone_b.0.1 + hailstone_b.1.1 * t2;
                    // t2 == (hailstone_a.0.1 - hailstone_b.0.1 + hailstone_a.1.1 * (hailstone_b.0.0 - hailstone_a.0.0) / hailstone_a.1.0) / (hailstone_b.1.1 - hailstone_a.1.1 * hailstone_b.1.0 / hailstone_a.1.0);
                    let top = (hailstone_a.1 .0 * hailstone_a.0 .1
                        - hailstone_a.1 .0 * hailstone_b.0 .1
                        + hailstone_a.1 .1 * (hailstone_b.0 .0 - hailstone_a.0 .0))
                        as f64;
                    let denominator = (hailstone_a.1 .0 * hailstone_b.1 .1
                        - hailstone_a.1 .1 * hailstone_b.1 .0)
                        as f64;
                    let t2 = if denominator == 0.0 {
                        if top != 0.0 {
                            return false;
                        }
                        0.0
                    } else {
                        top / denominator
                    };
                    let top = hailstone_b.0 .0 as f64 - hailstone_a.0 .0 as f64
                        + hailstone_b.1 .0 as f64 * t2;
                    let bottom = hailstone_a.1 .0 as f64;
                    let t1 = if bottom == 0.0 {
                        if top != 0.0 {
                            return false;
                        }
                        0.0
                    } else {
                        top / bottom
                    };
                    t1 >= 0.0
                        && t2 >= 0.0
                        && test_area
                            .contains(&(hailstone_a.0 .0 as f64 + hailstone_a.1 .0 as f64 * t1))
                        && test_area
                            .contains(&(hailstone_a.0 .1 as f64 + hailstone_a.1 .1 as f64 * t1))
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            2,
            sum(
                7..=27,
                "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3"
                    .as_bytes()
            )
        );
    }
}

fn main() {
    println!(
        "{}",
        sum(200000000000000..=400000000000000, std::io::stdin().lock())
    );
}
