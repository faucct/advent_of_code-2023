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
    for start_direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let mut prev_direction = start_direction;
        let mut position = start_direction.step(start);
        let mut cycle = std::collections::HashSet::new();
        loop {
            if let Some(tile) = map
                .get_mut(position.0)
                .and_then(|row| row.get_mut(position.1))
            {
                cycle.insert(position);
                if *tile as char == 'S' {
                    *tile = match prev_direction {
                        Direction::Down => match start_direction {
                            Direction::Left => 'J',
                            Direction::Right => 'L',
                            _ => '|',
                        },
                        Direction::Up => match start_direction {
                            Direction::Left => '7',
                            Direction::Right => 'F',
                            _ => '|',
                        },
                        Direction::Right => match start_direction {
                            Direction::Up => 'J',
                            Direction::Down => '7',
                            _ => '-',
                        },
                        Direction::Left => match start_direction {
                            Direction::Up => 'L',
                            Direction::Down => 'F',
                            _ => '-',
                        },
                    } as u8;
                    let mut filled = 0;
                    for (i, row) in map.iter_mut().enumerate() {
                        let mut inside = false;
                        for (j, tile) in row.iter_mut().enumerate() {
                            if cycle.contains(&(i, j)) && (*tile == '|' as u8 || *tile == '7' as u8 || *tile == 'F' as u8) {
                                inside = !inside;
                            }
                            if cycle.contains(&(i, j)) {
                                *tile = 'Z' as u8;
                            } else if inside {
                                *tile = 'X' as u8;
                                filled += 1;
                            }
                        }
                    }
                    return filled;
                } else if let Some(direction) = prev_direction.next(*tile as char) {
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
        assert_eq!(
            10,
            sum("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
