#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn sum(reader: impl std::io::BufRead) -> usize {
    let map = reader
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
    ([Direction::Up, Direction::Down, Direction::Left, Direction::Right].into_iter().map(|mut direction| {
        println!("hi");
        let mut steps = 1;
        let mut position = start;
        loop {
            println!("{:?} {:?}", position, direction);
            match direction {
                Direction::Up => if position.0 == 0 { return 0 } else {
                    position = (position.0 - 1, position.1);
                    direction = match map[position.0][position.1] as char {
                        'S' => return steps,
                        '|' => Direction::Up,
                        'F' => Direction::Right,
                        '7' => Direction::Left,
                        _ => return 0,
                    }
                },
                Direction::Down => if position.0 + 1 == map.len() { return 0 } else {
                    position = (position.0 + 1, position.1);
                    direction = match map[position.0][position.1] as char {
                        'S' => return steps,
                        '|' => Direction::Down,
                        'J' => Direction::Left,
                        'L' => Direction::Right,
                        _ => return 0,
                    }
                },
                Direction::Left => if position.1 == 0 { return 0 } else {
                    position = (position.0, position.1 - 1);
                    direction = match map[position.0][position.1] as char {
                        'S' => return steps,
                        '-' => Direction::Left,
                        'F' => Direction::Down,
                        'L' => Direction::Up,
                        _ => return 0,
                    }
                },
                Direction::Right => if position.1 + 1 == map[position.0].len() { return 0 } else {
                    position = (position.0, position.1 + 1);
                    direction = match map[position.0][position.1] as char {
                        'S' => return steps,
                        '-' => Direction::Right,
                        'J' => Direction::Up,
                        '7' => Direction::Down,
                        _ => return 0,
                    }
                },
            }
            steps += 1;
        }
    }).max().unwrap() + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            4,
            sum(".....
.S-7.
.|.|.
.L-J.
....."
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
