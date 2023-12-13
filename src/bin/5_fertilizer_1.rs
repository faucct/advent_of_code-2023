fn lowest_location(mut reader: impl std::io::BufRead) -> u64 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut seeds = line
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|seed| {
            seed.strip_suffix("\n")
                .unwrap_or(seed)
                .parse()
                .map_err(|_| seed)
                .unwrap()
        })
        .collect::<Vec<u64>>();
    line.clear();
    reader.read_line(&mut line).unwrap();
    for _ in 0..7 {
        seeds.sort();
        line.clear();
        reader.read_line(&mut line).unwrap();
        let mut seed_to_soil = Vec::new();
        loop {
            line.clear();
            reader.read_line(&mut line).unwrap();
            let line = line.strip_suffix("\n").unwrap_or(&line);
            if line.is_empty() {
                break;
            }
            let [destination_start, source_start, len]: [&str; 3] =
                line.split(" ").collect::<Vec<_>>().try_into().unwrap();
            seed_to_soil.push((
                source_start.parse::<u64>().unwrap(),
                (
                    len.parse::<u64>().unwrap(),
                    destination_start.parse::<u64>().unwrap(),
                ),
            ));
        }
        seed_to_soil.sort();
        'seed: for seed in &mut seeds {
            for entry in seed_to_soil[..seed_to_soil.partition_point(|entry| entry.0 <= *seed)]
                .iter()
                .rev()
            {
                if *seed < entry.0 + entry.1 .0 {
                    *seed = *seed - entry.0 + entry.1 .1;
                    continue 'seed;
                }
                break;
            }
        }
    }
    seeds.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            35,
            lowest_location(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
                    .as_bytes()
            )
        );
    }
}

fn main() {
    println!("{}", lowest_location(std::io::stdin().lock()));
}
