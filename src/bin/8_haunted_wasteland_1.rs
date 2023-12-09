fn sum(reader: impl std::io::BufRead) -> usize {
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
    let mut steps = 0;
    let mut node = "AAA";
    let mut instruction = instructions.chars();
    loop {
        if node == "ZZZ" {
            return steps;
        }
        if let Some(c) = instruction.next() {
            steps += 1;
            node = if c == 'L' { graph[node].0 } else { graph[node].1 };
        } else {
            instruction = instructions.chars();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            2,
            sum("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"
                .as_bytes())
        );
        assert_eq!(
            6,
            sum("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
