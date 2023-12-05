fn lowest_location(mut reader: impl std::io::BufRead) -> u64 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let seeds = line.strip_prefix("seeds: ").unwrap();
    let seeds = seeds.strip_suffix("\n").unwrap_or(seeds);
    let mut seeds = seeds
        .split(" ")
        .map(|seed| seed.parse::<u64>().map_err(|_| seed).unwrap());
    let mut seeds = std::iter::from_fn(|| {
        if let Some(start) = seeds.next() {
            let len = seeds.next().unwrap();
            Some(start..start + len)
        } else {
            None
        }
    })
    .collect::<Vec<_>>();
    line.clear();
    reader.read_line(&mut line).unwrap();
    for _ in 0..7 {
        seeds.sort_by_key(|seeds| seeds.start);
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
            let source_start = source_start.parse::<u64>().unwrap();
            seed_to_soil.push((
                source_start,
                source_start + len.parse::<u64>().unwrap(),
                destination_start.parse::<u64>().unwrap(),
            ));
        }
        seed_to_soil.sort();
        let mut new_seeds = Vec::new();
        'seed: for mut seed in seeds {
            for entry in seed_to_soil[..seed_to_soil.partition_point(|entry| entry.0 < seed.end)]
                .iter()
                .rev()
            {
                if seed.is_empty() {
                    break;
                }
                if seed.start < entry.1 {
                    if entry.0 < seed.end {
                        if entry.1 < seed.end {
                            new_seeds.push(entry.1..seed.end);
                            seed.end = entry.1;
                        }
                        if seed.start < entry.0 {
                            new_seeds.push(entry.2..seed.end + entry.2 - entry.0);
                            seed.end = entry.0;
                        } else {
                            new_seeds
                                .push(entry.2 + seed.start - entry.0..seed.end + entry.2 - entry.0);
                            continue 'seed;
                        }
                    }
                } else {
                    new_seeds.push(seed);
                    continue 'seed;
                }
            }
            if !seed.is_empty() {
                new_seeds.push(seed);
            }
        }
        seeds = new_seeds;
    }
    seeds.into_iter().map(|seeds| seeds.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            46,
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
