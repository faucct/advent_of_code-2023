fn sum(reader: impl std::io::BufRead) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        if a == 0 {
            b
        } else {
            gcd(b % a, a)
        }
    }

    let mut lines = reader.lines();
    let instructions = lines.next().unwrap().unwrap();
    lines.next().unwrap().unwrap();
    let mut graph = std::collections::HashMap::new();
    let lines = lines.map(|line| line.unwrap()).collect::<Vec<_>>();
    for line in &lines {
        let (from, to) = line.strip_suffix(")").unwrap().split_once(" = (").unwrap();
        let to = to.split_once(", ").unwrap();
        graph.insert(from, (to.0, to.1));
    }
    let nodes = graph
        .keys()
        .into_iter()
        .filter(|key| key.ends_with("A"))
        .map(|key| *key)
        .collect::<Vec<_>>();
    let nodes_zs = nodes.iter().map(|node| {
        let instruction = || {
            let instructions = &instructions;
            let mut instruction = instructions.chars();
            move || {
                loop {
                    if let Some(c) = instruction.next() {
                        return c == 'L';
                    } else {
                        instruction = instructions.chars();
                    }
                }
            }
        };
        let mut steps = 0;
        let mut fast_node = *node;
        let mut fast_instruction = instruction();
        let mut slow_node = *node;
        let mut slow_instruction = instruction();
        loop {
            for _ in 0..2 {
                fast_node = if fast_instruction() {
                    graph[fast_node].0
                } else {
                    graph[fast_node].1
                };
            }
            slow_node = if slow_instruction() {
                graph[slow_node].0
            } else {
                graph[slow_node].1
            };
            steps += 1;
            if fast_node == slow_node && steps % instructions.len() == 0 {
                break;
            }
        }
        let mut slow_node = *node;
        let mut slow_instruction = instruction();
        while slow_node != fast_node {
            fast_node = if fast_instruction() {
                graph[fast_node].0
            } else {
                graph[fast_node].1
            };
            slow_node = if slow_instruction() {
                graph[slow_node].0
            } else {
                graph[slow_node].1
            };
        }
        let mut loop_start = 0;
        let mut zs_before_loop = Vec::new();
        let mut fast_node = *node;
        let mut fast_instruction = instruction();
        while fast_node != slow_node {
            if fast_node.ends_with("Z") {
                zs_before_loop.push(loop_start);
            }
            fast_node = if fast_instruction() {
                graph[fast_node].0
            } else {
                graph[fast_node].1
            };
            loop_start += 1;
        }
        let mut loop_length = 0;
        let mut zs_after_loop = Vec::new();
        loop {
            fast_node = if fast_instruction() {
                graph[fast_node].0
            } else {
                graph[fast_node].1
            };
            loop_length += 1;
            if fast_node == slow_node && loop_length % instructions.len() == 0 {
                break;
            }
            if fast_node.ends_with("Z") {
                zs_after_loop.push(loop_length);
            }
        }
        (zs_before_loop, loop_start, zs_after_loop, loop_length)
    });
    let mut left = (Vec::new(), 0, (0..instructions.len()).collect::<Vec<_>>(), instructions.len());
    for right in nodes_zs {
        let loop_start = left.1.max(right.1);
        let loop_length = left.3 * right.3 / gcd(left.3, right.3);

        let mut left_looped = left.1;
        let mut lefts = left.0.iter().copied().chain(std::iter::once(()).cycle().flat_map(|_| {
            left_looped += left.3;
            left.2.iter().map(move |i| i + left_looped - left.3)
        })).peekable();
        let mut right_looped = right.1;
        let mut rights = right.0.iter().copied().chain(std::iter::once(()).cycle().flat_map(|_| {
            right_looped += right.3;
            right.2.iter().map(move |i| i + right_looped - right.3)
        })).peekable();
        let mut zs_before_loop = Vec::new();
        loop {
            let left = lefts.peek().unwrap();
            let right = rights.peek().unwrap();
            if left < right {
                lefts.next();
            } else if right < left {
                rights.next();
            } else if *left < loop_start {
                zs_before_loop.push(*left);
                lefts.next();
                rights.next();
            } else {
                break;
            }
        }
        let mut zs_after_loop = Vec::new();
        loop {
            let left = lefts.peek().unwrap() - loop_start;
            let right = rights.peek().unwrap() - loop_start;
            if left < right {
                lefts.next();
            } else if right < left {
                rights.next();
            } else if left < loop_length {
                zs_after_loop.push(left);
                lefts.next();
                rights.next();
            } else {
                break;
            }
        }
        left = (zs_before_loop, loop_start, zs_after_loop, loop_length);
    }
    if let Some(i) = left.0.first() {
        *i
    } else {
        left.1 + *left.2.first().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            6,
            sum("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"
            .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
