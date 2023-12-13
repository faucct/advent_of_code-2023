use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn mirror(pattern: &Vec<(Vec<u8>, u64)>, original: usize) -> usize {
    'mirror: for mirror in 0..pattern.len() - 1 {
        if mirror + 1 == original {
            continue;
        }
        for reflection in 0..pattern.len() {
            let up = if let Some(up) = pattern.get(mirror.wrapping_sub(reflection)) {
                up.1
            } else {
                break;
            };
            let down = if let Some(down) = pattern.get(mirror + 1 + reflection) {
                down.1
            } else {
                break;
            };
            if up != down {
                continue 'mirror;
            }
        }
        for reflection in 0..pattern.len() {
            let up = if let Some(up) = pattern.get(mirror.wrapping_sub(reflection)) {
                &up.0
            } else {
                break;
            };
            let down = if let Some(down) = pattern.get(mirror + 1 + reflection) {
                &down.0
            } else {
                break;
            };
            if up != down {
                continue 'mirror;
            }
        }
        return mirror + 1;
    }
    0
}

fn sum(reader: impl std::io::BufRead) -> usize {
    let mut pattern: Vec<(Vec<u8>, u64)> = Vec::new();
    let mut lines = reader.lines();
    let patterns = std::iter::from_fn(|| {
        while let Some(line) = lines.next() {
            let line = line.unwrap();
            if line.is_empty() {
                break;
            } else {
                let line = line.into_bytes();
                let mut hasher = DefaultHasher::new();
                line.hash(&mut hasher);
                pattern.push((line, hasher.finish()));
            }
        }
        if pattern.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut pattern))
        }
    });
    patterns
        .map(|mut pattern| {
            for scale in [100, 1] {
                let original_mirror: usize = mirror(&pattern, 0);
                for i in 0..pattern.len() {
                    for j in 0..pattern[i].0.len() {
                        pattern[i].0[j] = if pattern[i].0[j] == '#' as u8 {
                            '.' as u8
                        } else {
                            '#' as u8
                        };
                        let mut hasher = DefaultHasher::new();
                        pattern[i].0.hash(&mut hasher);
                        pattern[i].1 = hasher.finish();
                        let mirror = mirror(&pattern, original_mirror);
                        if mirror != 0 && mirror != original_mirror {
                            return scale * mirror;
                        }
                        pattern[i].0[j] = if pattern[i].0[j] == '#' as u8 {
                            '.' as u8
                        } else {
                            '#' as u8
                        };
                        let mut hasher = DefaultHasher::new();
                        pattern[i].0.hash(&mut hasher);
                        pattern[i].1 = hasher.finish();
                    }
                }
                pattern = (0..pattern[0].0.len())
                    .map(|i| {
                        let line = pattern.iter().map(|row| row.0[i]).collect::<Vec<_>>();
                        let mut hasher = DefaultHasher::new();
                        line.hash(&mut hasher);
                        (line, hasher.finish())
                    })
                    .collect::<Vec<_>>();
            }
            panic!();
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            400,
            sum("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
