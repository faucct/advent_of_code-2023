fn next(map: &mut Vec<Vec<u8>>) {
    for column in 0..map[0].len() {
        let mut free_position = 0;
        for row in 0..map.len() {
            match map[row][column] as char {
                'O' => {
                    let tmp = map[free_position][column];
                    map[free_position][column] = map[row][column];
                    map[row][column] = tmp;
                    free_position += 1;
                }
                '.' => {}
                '#' => free_position = row + 1,
                _ => panic!(),
            }
        }
    }
    for row in 0..map.len() {
        let mut free_position = 0;
        for column in 0..map[0].len() {
            match map[row][column] as char {
                'O' => {
                    let tmp = map[row][free_position];
                    map[row][free_position] = map[row][column];
                    map[row][column] = tmp;
                    free_position += 1;
                }
                '.' => {}
                '#' => free_position = column + 1,
                _ => panic!(),
            }
        }
    }
    for column in (0..map[0].len()).rev() {
        let mut free_position = map.len() - 1;
        for row in (0..map.len()).rev() {
            match map[row][column] as char {
                'O' => {
                    let tmp = map[free_position][column];
                    map[free_position][column] = map[row][column];
                    map[row][column] = tmp;
                    free_position = free_position.wrapping_sub(1);
                }
                '.' => {}
                '#' => free_position = row.wrapping_sub(1),
                _ => panic!(),
            }
        }
    }
    for row in (0..map.len()).rev() {
        let mut free_position = map[0].len() - 1;
        for column in (0..map[0].len()).rev() {
            match map[row][column] as char {
                'O' => {
                    let tmp = map[row][free_position];
                    map[row][free_position] = map[row][column];
                    map[row][column] = tmp;
                    free_position = free_position.wrapping_sub(1);
                }
                '.' => {}
                '#' => free_position = column.wrapping_sub(1),
                _ => panic!(),
            }
        }
    }
}

fn sum(reader: impl std::io::BufRead) -> usize {
    let mut maps_cycles = std::collections::HashMap::<Vec<Vec<u8>>, Vec<usize>>::new();
    let mut map = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect::<Vec<_>>();
    const CYCLES: usize = 1000000000;
    maps_cycles.entry(map.clone()).or_default().push(0);
    'cycle: for cycle in 0..CYCLES {
        next(&mut map);
        let cycles = maps_cycles.entry(map.clone()).or_default();
        for prev in cycles.iter() {
            if (cycle + 1 - prev) % 4 == 0 {
                for _ in 0..(CYCLES - cycle - 1) % (cycle + 1 - prev) {
                    next(&mut map);
                }
                break 'cycle;
            }
        }
        cycles.push(cycle + 1);
    }
    let mut sum = 0;
    for column in 0..map[0].len() {
        for row in 0..map.len() {
            match map[row][column] as char {
                'O' => {
                    sum += map.len() - row;
                }
                '.' => {}
                '#' => {}
                _ => panic!(),
            }
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            64,
            sum("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
