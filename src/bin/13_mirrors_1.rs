use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn sum(reader: impl std::io::BufRead) -> usize {
    let mut pattern: Vec<(String, u64)> = Vec::new();
    let mut lines = reader.lines();
    let patterns = std::iter::from_fn(|| {
        while let Some(line) = lines.next() {
            let line = line.unwrap();
            if line.is_empty() {
                break;
            } else {
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
        .map(|pattern| {
            'mirror: for mirror in 0..pattern.len() - 1 {
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
                return 100 * (mirror + 1);
            }
            'mirror: for mirror in 0..pattern[0].0.len() - 1 {
                for (row, _) in &pattern {
                    for reflection in 0..row.len() {
                        let up =
                            if let Some(up) = row.as_bytes().get(mirror.wrapping_sub(reflection)) {
                                up
                            } else {
                                break;
                            };
                        let down = if let Some(down) = row.as_bytes().get(mirror + 1 + reflection) {
                            down
                        } else {
                            break;
                        };
                        if up != down {
                            continue 'mirror;
                        }
                    }
                }
                return mirror + 1;
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
            405,
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
