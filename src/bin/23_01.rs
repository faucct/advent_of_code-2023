fn sum(reader: impl std::io::BufRead) -> usize {
    let map = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect::<Vec<_>>();
    fn rec(
        map: &Vec<Vec<u8>>,
        visited: &mut Vec<Vec<bool>>,
        steps: usize,
        position: (usize, usize),
    ) -> usize {
        if let Some(&tile) = map.get(position.0).and_then(|row| row.get(position.1)) {
            if std::mem::replace(&mut visited[position.0][position.1], true) {
                return 0;
            };
            let steps = if tile == '#' as u8 {
                0
            } else if tile == '>' as u8 {
                rec(
                    map,
                    visited,
                    steps + 1,
                    (position.0, position.1.wrapping_add(1)),
                )
            } else if tile == 'v' as u8 {
                rec(
                    map,
                    visited,
                    steps + 1,
                    (position.0.wrapping_add(1), position.1),
                )
            } else if tile == '<' as u8 {
                rec(
                    map,
                    visited,
                    steps + 1,
                    (position.0, position.1.wrapping_sub(1)),
                )
            } else if tile == '^' as u8 {
                rec(
                    map,
                    visited,
                    steps + 1,
                    (position.0.wrapping_sub(1), position.1),
                )
            } else if position.0 + 1 == map.len() {
                steps
            } else {
                rec(
                    map,
                    visited,
                    steps + 1,
                    (position.0, position.1.wrapping_add(1)),
                )
                .max(rec(
                    map,
                    visited,
                    steps + 1,
                    (position.0.wrapping_add(1), position.1),
                ))
                .max(rec(
                    map,
                    visited,
                    steps + 1,
                    (position.0, position.1.wrapping_sub(1)),
                ))
                .max(rec(
                    map,
                    visited,
                    steps + 1,
                    (position.0.wrapping_sub(1), position.1),
                ))
            };
            visited[position.0][position.1] = false;
            steps
        } else {
            0
        }
    }
    let mut visited = map
        .iter()
        .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    rec(
        &map,
        &mut visited,
        0,
        (
            0,
            map[0].iter().position(|&tile| tile == '.' as u8).unwrap(),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            94,
            sum("#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
