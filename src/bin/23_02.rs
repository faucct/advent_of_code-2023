use std::collections::{BTreeSet, HashMap};

fn sum(reader: impl std::io::BufRead) -> usize {
    let map = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect::<Vec<_>>();
    let start = map[0].iter().position(|&tile| tile == '.' as u8).unwrap() as u8;
    let mut nodes = HashMap::new();
    nodes.insert((0, start), HashMap::<(u8, u8), usize>::new());
    let mut queue = Vec::new();
    queue.push(((0u8, start), (0u8, start), (0u8, start), 0));
    while let Some((mut from, prev, position, mut steps)) = queue.pop() {
        let positions = [
            (position.0, position.1.wrapping_add(1)),
            (position.0.wrapping_add(1), position.1),
            (position.0, position.1.wrapping_sub(1)),
            (position.0.wrapping_sub(1), position.1),
        ];
        let positions = positions.into_iter().filter(|position| {
            if let Some(&tile) = map
                .get(position.0 as usize)
                .and_then(|row| row.get(position.1 as usize))
            {
                tile != '#' as u8
            } else {
                false
            }
        });
        if position != from && positions.clone().count() != 2 {
            let entry = nodes.entry(from).or_default().entry(position).or_default();
            *entry = (*entry).max(steps);
            if nodes.contains_key(&position) {
                nodes
                    .entry(position)
                    .or_default()
                    .entry(from)
                    .and_modify(|entry| *entry = steps.max(*entry))
                    .or_insert(steps);
                continue;
            }
            nodes
                .entry(position)
                .or_default()
                .entry(from)
                .and_modify(|entry| *entry = steps.max(*entry))
                .or_insert(steps);
            from = position;
            steps = 0;
        }
        for next in positions {
            if next != prev {
                queue.push((from, position, next, steps + 1));
            }
        }
    }

    fn dfs(
        end: u8,
        graph: &HashMap<(u8, u8), HashMap<(u8, u8), usize>>,
        visited: &mut BTreeSet<(u8, u8)>,
        position: (u8, u8),
    ) -> bool {
        if position.0 == end {
            return true;
        }
        if !visited.insert(position) {
            return false;
        }
        for &next in graph[&position].keys() {
            if dfs(end, graph, visited, next) {
                return true;
            }
        }
        false
    }

    fn rec(
        end: u8,
        graph: &HashMap<(u8, u8), HashMap<(u8, u8), usize>>,
        visited: &mut BTreeSet<(u8, u8)>,
        steps: usize,
        position: (u8, u8),
    ) -> usize {
        if position.0 == end {
            return steps;
        }
        if !dfs(end, graph, &mut visited.clone(), position) {
            return 0;
        }
        if !visited.insert(position) {
            return 0;
        }
        let steps = graph[&position]
            .iter()
            .map(|edge| rec(end, graph, visited, steps + edge.1, *edge.0))
            .max()
            .unwrap_or(0);
        visited.remove(&position);
        steps
    }
    rec(
        map.len() as u8 - 1,
        &nodes,
        &mut Default::default(),
        0,
        (0, start),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            154,
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
