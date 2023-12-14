fn sum(reader: impl std::io::BufRead) -> usize {
    let mut sum = 0;
    let map = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect::<Vec<_>>();
    for column in 0..map[0].len() {
        let mut free_position = 0;
        for row in 0..map.len() {
            match map[row][column] as char {
                'O' => {
                    sum += map.len() - free_position;
                    free_position += 1;
                }
                '.' => {}
                '#' => free_position = row + 1,
                _ => panic!(),
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            136,
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
