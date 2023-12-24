use std::fmt::Write;

fn escape(string: String) -> String {
    string
        .replace("+", "%2B")
        .replace(" ", "+")
        .replace("/", "%2F")
        .replace("(", "%28")
        .replace(")", "%29")
        .replace("=", "%3D")
}

fn sum(reader: impl std::io::BufRead) -> String {
    let hailstones = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut line = line.split(" @ ").map(|line| {
                let mut line = line
                    .split(", ")
                    .map(|number| number.parse::<i64>().unwrap());
                std::array::from_fn::<i64, 3, _>(|_| line.next().unwrap())
            });
            (line.next().unwrap(), line.next().unwrap())
        })
        .take(3)
        .collect::<Vec<_>>();
    let mut url = String::new();
    write!(
        &mut url,
        "https://www.wolframalpha.com/input?i=system+equation+calculator&assumption=%22FSelect%22+-%3E+%7B%7B%22SolveSystemOf4EquationsCalculator%22%7D%2C+%22dflt%22%7D"
    )
    .unwrap();
    for i in 0..3 {
        let equation = format!(
            "(({} + a * {}) * (b - c) - ({} + b * {}) * (a - c)) / (b - a) = {} + c * {}",
            hailstones[0].0[i],
            hailstones[0].1[i],
            hailstones[1].0[i],
            hailstones[1].1[i],
            hailstones[2].0[i],
            hailstones[2].1[i],
        );
        write!(
            &mut url,
            "&assumption=%7B%22F%22%2C+%22SolveSystemOf4EquationsCalculator%22%2C+%22equation{}%22%7D+-%3E%22{}%22",
            i + 1,
            escape(equation),
        ).unwrap();
    }
    let equation = format!(
        "s=(({} + a * {}) * b - ({} + b * {}) * a) / (b - a)",
        hailstones[0].0.into_iter().sum::<i64>(),
        hailstones[0].1.into_iter().sum::<i64>(),
        hailstones[1].0.into_iter().sum::<i64>(),
        hailstones[1].1.into_iter().sum::<i64>(),
    );
    write!(
        &mut url,
        "&assumption=%7B%22F%22%2C+%22SolveSystemOf4EquationsCalculator%22%2C+%22equation4%22%7D+-%3E%22{}%22",
        escape(equation),
    ).unwrap();
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            "https://www.wolframalpha.com/input?i=system+equation+calculator&assumption=%22FSelect%22+-%3E+%7B%7B%22SolveSystemOf4EquationsCalculator%22%7D%2C+%22dflt%22%7D&assumption=%7B%22F%22%2C+%22SolveSystemOf4EquationsCalculator%22%2C+%22equation1%22%7D+-%3E%22%28%2819+%2B+a+*+-2%29+*+%28b+-+c%29+-+%2818+%2B+b+*+-1%29+*+%28a+-+c%29%29+%2F+%28b+-+a%29+%3D+20+%2B+c+*+-2%22&assumption=%7B%22F%22%2C+%22SolveSystemOf4EquationsCalculator%22%2C+%22equation2%22%7D+-%3E%22%28%2813+%2B+a+*+1%29+*+%28b+-+c%29+-+%2819+%2B+b+*+-1%29+*+%28a+-+c%29%29+%2F+%28b+-+a%29+%3D+25+%2B+c+*+-2%22&assumption=%7B%22F%22%2C+%22SolveSystemOf4EquationsCalculator%22%2C+%22equation3%22%7D+-%3E%22%28%2830+%2B+a+*+-2%29+*+%28b+-+c%29+-+%2822+%2B+b+*+-2%29+*+%28a+-+c%29%29+%2F+%28b+-+a%29+%3D+34+%2B+c+*+-4%22&assumption=%7B%22F%22%2C+%22SolveSystemOf4EquationsCalculator%22%2C+%22equation4%22%7D+-%3E%22s%3D%28%2862+%2B+a+*+-3%29+*+b+-+%2859+%2B+b+*+-4%29+*+a%29+%2F+%28b+-+a%29%22".to_string(),
            sum(
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
    println!("{}", sum(std::io::stdin().lock()));
}
