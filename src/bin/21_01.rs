fn sum(goal: usize, reader: impl std::io::BufRead) -> usize {
    let map = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect::<Vec<_>>();
    let mut visited = map
        .iter()
        .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut even_count = 0;
    let mut queue = std::collections::VecDeque::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' as u8 {
                visited[i][j] = true;
                even_count += 1;
                queue.push_back((i, j));
            }
        }
    }
    let mut steps = 1;
    let mut steps_queue_len = queue.len();
    while let Some((i, j)) = queue.pop_front() {
        steps_queue_len -= 1;
        let mut visit = |i: usize, j: usize| {
            if let Some(row) = map.get(i) {
                if let Some(&cell) = row.get(j) {
                    if cell != '#' as u8 && !std::mem::replace(&mut visited[i][j], true) {
                        queue.push_back((i, j));
                        if steps % 2 == 0 {
                            even_count += 1;
                        }
                    }
                }
            }
        };
        visit(i, j.wrapping_sub(1));
        visit(i, j.wrapping_add(1));
        visit(i.wrapping_sub(1), j);
        visit(i.wrapping_add(1), j);
        if steps_queue_len == 0 {
            if steps % 2 == 0 {
                println!("{} {}", steps, even_count);
                for row in &visited {
                    for &cell in row {
                        print!("{}", if cell { 'O' } else { '.' });
                    }
                    println!("");
                }
            }
            println!("");
            if steps == goal {
                return even_count;
            }
            steps += 1;
            steps_queue_len = queue.len();
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            16,
            sum(
                6,
                "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
                    .as_bytes()
            )
        );
    }
}

fn main() {
    println!("{}", sum(64, std::io::stdin().lock()));
}
