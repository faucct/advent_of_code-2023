#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (position.0.wrapping_sub(1), position.1),
            Direction::Down => (position.0.wrapping_add(1), position.1),
            Direction::Left => (position.0, position.1.wrapping_sub(1)),
            Direction::Right => (position.0, position.1.wrapping_add(1)),
        }
    }

    fn next(self, tile: char) -> Option<Self> {
        Some(match self {
            Direction::Up => match tile {
                '|' => Direction::Up,
                'F' => Direction::Right,
                '7' => Direction::Left,
                _ => return None,
            },
            Direction::Down => match tile {
                '|' => Direction::Down,
                'J' => Direction::Left,
                'L' => Direction::Right,
                _ => return None,
            },
            Direction::Left => match tile {
                '-' => Direction::Left,
                'F' => Direction::Down,
                'L' => Direction::Up,
                _ => return None,
            },
            Direction::Right => match tile {
                '-' => Direction::Right,
                'J' => Direction::Up,
                '7' => Direction::Down,
                _ => return None,
            },
        })
    }

    fn clockwise(&self, direction: Direction) -> i8 {
        match self {
            Direction::Up => match direction {
                Direction::Left => -1,
                Direction::Right => 1,
                _ => 0,
            },
            Direction::Down => match direction {
                Direction::Left => 1,
                Direction::Right => -1,
                _ => 0,
            },
            Direction::Left => match direction {
                Direction::Up => 1,
                Direction::Down => -1,
                _ => 0,
            },
            Direction::Right => match direction {
                Direction::Down => 1,
                Direction::Up => -1,
                _ => 0,
            },
        }
    }
}

fn sum(reader: impl std::io::BufRead) -> usize {
    let mut map = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect::<Vec<_>>();
    let start = map
        .iter()
        .enumerate()
        .filter_map(|row| {
            row.1
                .iter()
                .position(|cell| *cell == 'S' as u8)
                .map(|position| (row.0, position))
        })
        .next()
        .unwrap();
    fn fill(
        map: &mut Vec<Vec<u8>>,
        cycle: &std::collections::HashSet<(usize, usize)>,
        position: (usize, usize),
    ) -> usize {
        if cycle.contains(&position) {
            return 0;
        }
        let mut filled = 0;
        if let Some(tile) = map
            .get_mut(position.0)
            .and_then(|row| row.get_mut(position.1))
        {
            if *tile != '0' as u8 && *tile != 'I' as u8 {
                *tile = if *tile == '.' as u8 {
                    filled += 1;
                    'I' as u8
                } else {
                    '0' as u8
                };
                filled += fill(map, cycle, (position.0, position.1.wrapping_add(1)));
                filled += fill(map, cycle, (position.0, position.1.wrapping_sub(1)));
                filled += fill(map, cycle, (position.0.wrapping_add(1), position.1));
                filled += fill(
                    map,
                    cycle,
                    (position.0.wrapping_add(1), position.1.wrapping_add(1)),
                );
                filled += fill(
                    map,
                    cycle,
                    (position.0.wrapping_add(1), position.1.wrapping_sub(1)),
                );
                filled += fill(map, cycle, (position.0.wrapping_sub(1), position.1));
                filled += fill(
                    map,
                    cycle,
                    (position.0.wrapping_sub(1), position.1.wrapping_add(1)),
                );
                filled += fill(
                    map,
                    cycle,
                    (position.0.wrapping_sub(1), position.1.wrapping_sub(1)),
                );
            }
        }
        filled
    }
    for start_direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let mut prev_direction = start_direction;
        let mut position = start_direction.step(start);
        let mut clockwise = 0;
        let mut cycle = std::collections::HashSet::new();
        let mut filled = 0;
        loop {
            if let Some(tile) = map
                .get_mut(position.0)
                .and_then(|row| row.get_mut(position.1))
            {
                cycle.insert(position);
                if *tile as char == 'S' {
                    let mut prev_direction = start_direction;
                    let mut position = start_direction.step(start);
                    loop {
                        if let Some(tile) = map
                            .get_mut(position.0)
                            .and_then(|row| row.get_mut(position.1))
                        {
                            let tile = *tile as char;
                            let direction = if tile == 'S' {
                                start_direction
                            } else {
                                prev_direction.next(tile).unwrap()
                            };
                            if prev_direction.clockwise(direction) * clockwise < 0 {
                                filled += match (prev_direction, direction) {
                                    (Direction::Up, Direction::Left)
                                    | (Direction::Left, Direction::Up) => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0.wrapping_add(1), position.1.wrapping_add(1)),
                                    ),
                                    (Direction::Up, Direction::Right)
                                    | (Direction::Right, Direction::Up) => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0.wrapping_add(1), position.1.wrapping_sub(1)),
                                    ),
                                    (Direction::Down, Direction::Left)
                                    | (Direction::Left, Direction::Down) => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0.wrapping_sub(1), position.1.wrapping_add(1)),
                                    ),
                                    (Direction::Down, Direction::Right)
                                    | (Direction::Right, Direction::Down) => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0.wrapping_sub(1), position.1.wrapping_sub(1)),
                                    ),
                                    _ => 0,
                                };
                                filled += if clockwise > 0 {
                                    match direction {
                                        Direction::Up => fill(
                                            &mut map,
                                            &cycle,
                                            (position.0, position.1.wrapping_add(1)),
                                        ),
                                        Direction::Down => fill(
                                            &mut map,
                                            &cycle,
                                            (position.0, position.1.wrapping_sub(1)),
                                        ),
                                        Direction::Left => fill(
                                            &mut map,
                                            &cycle,
                                            (position.0.wrapping_sub(1), position.1),
                                        ),
                                        Direction::Right => fill(
                                            &mut map,
                                            &cycle,
                                            (position.0.wrapping_add(1), position.1),
                                        ),
                                    }
                                } else {
                                    match prev_direction {
                                        Direction::Down => fill(
                                            &mut map,
                                            &cycle,
                                            (position.0, position.1.wrapping_add(1)),
                                        ),
                                        Direction::Up => fill(
                                            &mut map,
                                            &cycle,
                                            (position.0, position.1.wrapping_sub(1)),
                                        ),
                                        Direction::Right => fill(
                                            &mut map,
                                            &cycle,
                                            (position.0.wrapping_sub(1), position.1),
                                        ),
                                        Direction::Left => fill(
                                            &mut map,
                                            &cycle,
                                            (position.0.wrapping_add(1), position.1),
                                        ),
                                    }
                                };
                            }
                            filled += if clockwise > 0 {
                                match prev_direction {
                                    Direction::Up => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0, position.1.wrapping_add(1)),
                                    ),
                                    Direction::Down => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0, position.1.wrapping_sub(1)),
                                    ),
                                    Direction::Left => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0.wrapping_sub(1), position.1),
                                    ),
                                    Direction::Right => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0.wrapping_add(1), position.1),
                                    ),
                                }
                            } else {
                                match prev_direction {
                                    Direction::Down => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0, position.1.wrapping_add(1)),
                                    ),
                                    Direction::Up => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0, position.1.wrapping_sub(1)),
                                    ),
                                    Direction::Right => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0.wrapping_sub(1), position.1),
                                    ),
                                    Direction::Left => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0.wrapping_add(1), position.1),
                                    ),
                                }
                            };
                            filled += if clockwise > 0 {
                                match direction {
                                    Direction::Up => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0, position.1.wrapping_add(1)),
                                    ),
                                    Direction::Down => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0, position.1.wrapping_sub(1)),
                                    ),
                                    Direction::Left => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0.wrapping_sub(1), position.1),
                                    ),
                                    Direction::Right => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0.wrapping_add(1), position.1),
                                    ),
                                }
                            } else {
                                match direction {
                                    Direction::Down => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0, position.1.wrapping_add(1)),
                                    ),
                                    Direction::Up => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0, position.1.wrapping_sub(1)),
                                    ),
                                    Direction::Right => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0.wrapping_sub(1), position.1),
                                    ),
                                    Direction::Left => fill(
                                        &mut map,
                                        &cycle,
                                        (position.0.wrapping_add(1), position.1),
                                    ),
                                }
                            };
                            if tile == 'S' {
                                for row in map {
                                    println!("{}", String::from_utf8(row).unwrap());
                                }
                                return filled;
                            }
                            prev_direction = direction;
                            position = direction.step(position);
                        }
                    }
                } else if let Some(direction) = prev_direction.next(*tile as char) {
                    clockwise += prev_direction.clockwise(direction);
                    prev_direction = direction;
                    position = direction.step(position);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            4,
            sum("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
                .as_bytes())
        );
        assert_eq!(
            8,
            sum(".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
