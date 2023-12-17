fn sum(reader: impl std::io::BufRead) -> u64 {
    let mut map = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| (c.to_digit(10).unwrap() as u8, [u64::MAX; 4]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut queue = std::collections::BinaryHeap::new();
    queue.push((std::cmp::Reverse(0u64), 0usize, 0usize, 0u8));
    queue.push((std::cmp::Reverse(0u64), 0usize, 0usize, 1u8));
    while let Some((std::cmp::Reverse(sum), i, j, direction)) = queue.pop() {
        if i + 1 == map.len() && j + 1 == map[i].len() {
            return sum;
        }
        let mut sum = sum;
        let mut i = i;
        let mut j = j;
        for _ in 0..3 {
            let directions = match direction {
                0 => {
                    j = j.wrapping_add(1);
                    [3, 1]
                }
                1 => {
                    i = i.wrapping_add(1);
                    [0, 2]
                }
                2 => {
                    j = j.wrapping_sub(1);
                    [1, 3]
                }
                3 => {
                    i = i.wrapping_sub(1);
                    [2, 0]
                }
                _ => panic!(),
            };
            if let Some(row) = map.get_mut(i) {
                if let Some(cell) = row.get_mut(j) {
                    sum += cell.0 as u64;
                    for direction in directions {
                        if sum < cell.1[direction as usize] {
                            cell.1[direction as usize] = sum;
                            queue.push((std::cmp::Reverse(sum), i, j, direction));
                        }
                    }
                    continue;
                }
            }
            break;
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
            102,
            sum("2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
