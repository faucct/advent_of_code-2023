fn sum(reader: impl std::io::BufRead) -> usize {
    let mut map = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .into_bytes()
                .into_iter()
                .map(|cell| (cell, [false; 4]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = map.len();
    let width = map[0].len();
    let mut queue = Vec::new();
    (0..map[0].len())
        .flat_map(|i| [(0, i, 1u8), (height - 1, i, 3u8)].into_iter())
        .chain((0..map.len()).flat_map(|i| [(i, 0, 0u8), (i, width - 1, 2u8)].into_iter()))
        .map(|start| {
            for row in &mut map {
                for cell in row {
                    cell.1 = [false; 4];
                }
            }
            queue.push(start);
            while let Some((i, j, direction)) = queue.pop() {
                if let Some(row) = map.get_mut(i) {
                    if let Some(cell) = row.get_mut(j) {
                        if std::mem::replace(&mut cell.1[direction as usize], true) {
                            continue;
                        }
                        match cell.0 as char {
                            '|' => {
                                if direction % 2 == 0 {
                                    queue.push((i.wrapping_sub(1), j, 3));
                                    queue.push((i.wrapping_add(1), j, 1));
                                } else if direction / 2 == 0 {
                                    queue.push((i.wrapping_add(1), j, direction));
                                } else {
                                    queue.push((i.wrapping_sub(1), j, direction));
                                }
                            }
                            '-' => {
                                if direction % 2 == 1 {
                                    queue.push((i, j.wrapping_add(1), 0));
                                    queue.push((i, j.wrapping_sub(1), 2));
                                } else if direction / 2 == 0 {
                                    queue.push((i, j.wrapping_add(1), direction));
                                } else {
                                    queue.push((i, j.wrapping_sub(1), direction));
                                }
                            }
                            '/' => match direction {
                                0 => queue.push((i.wrapping_sub(1), j, 3)),
                                1 => queue.push((i, j.wrapping_sub(1), 2)),
                                2 => queue.push((i.wrapping_add(1), j, 1)),
                                3 => queue.push((i, j.wrapping_add(1), 0)),
                                _ => panic!(),
                            },
                            '\\' => match direction {
                                0 => queue.push((i.wrapping_add(1), j, 1)),
                                1 => queue.push((i, j.wrapping_add(1), 0)),
                                2 => queue.push((i.wrapping_sub(1), j, 3)),
                                3 => queue.push((i, j.wrapping_sub(1), 2)),
                                _ => panic!(),
                            },
                            '.' => match direction {
                                0 => queue.push((i, j.wrapping_add(1), 0)),
                                1 => queue.push((i.wrapping_add(1), j, 1)),
                                2 => queue.push((i, j.wrapping_sub(1), 2)),
                                3 => queue.push((i.wrapping_sub(1), j, 3)),
                                _ => panic!(),
                            },
                            cell => panic!("{:?}", cell),
                        }
                    }
                }
            }
            map.iter()
                .flat_map(|row| {
                    row.iter()
                        .filter(|cell| cell.1.iter().any(|&energized| energized))
                })
                .count()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            51,
            sum(".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
