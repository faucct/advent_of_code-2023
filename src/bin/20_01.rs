fn sum(reader: impl std::io::BufRead) -> usize {
    let mut flip_flops = std::collections::HashMap::new();
    let mut conjunctions = std::collections::HashMap::new();
    let mut broadcasters = std::collections::HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(flip_flop) = line.strip_prefix("%") {
            let (name, destinations) = flip_flop.split_once(" -> ").unwrap();
            flip_flops.insert(
                name.to_string(),
                (
                    destinations
                        .split(", ")
                        .map(|destination| destination.to_string())
                        .collect::<Vec<_>>(),
                    std::cell::Cell::new(false),
                ),
            );
        } else if let Some(conjunction) = line.strip_prefix("&") {
            let (name, destinations) = conjunction.split_once(" -> ").unwrap();
            conjunctions.insert(
                name.to_string(),
                (
                    destinations
                        .split(", ")
                        .map(|destination| destination.to_string())
                        .collect::<Vec<_>>(),
                    std::cell::RefCell::new(std::collections::HashSet::<&str>::new()),
                ),
            );
        } else {
            let (name, destinations) = line.split_once(" -> ").unwrap();
            broadcasters.insert(
                name.to_string(),
                destinations
                    .split(", ")
                    .map(|destination| destination.to_string())
                    .collect::<Vec<_>>(),
            );
        }
    }
    for (input, (destinations, _)) in &flip_flops {
        for destination in destinations {
            if let Some((_, destination)) = conjunctions.get(destination) {
                destination.borrow_mut().insert(input);
            }
        }
    }
    for (input, (destinations, _)) in &conjunctions {
        for destination in destinations {
            if let Some((_, destination)) = conjunctions.get(destination) {
                destination.borrow_mut().insert(input);
            }
        }
    }
    for (input, destinations) in &broadcasters {
        for destination in destinations {
            if let Some((_, destination)) = conjunctions.get(destination) {
                destination.borrow_mut().insert(input);
            }
        }
    }
    let mut lows = 0;
    let mut highs = 0;
    let mut queue = std::collections::VecDeque::new();
    for _ in 0..1000 {
        queue.push_back(("button", false, "broadcaster"));
        while let Some((input, high, output)) = queue.pop_front() {
            if high {
                highs += 1;
            } else {
                lows += 1;
            }
            if let Some(flip_flop) = flip_flops.get(output) {
                if !high {
                    flip_flop.1.set(!flip_flop.1.take());
                    for destination in &flip_flop.0 {
                        queue.push_back((output, flip_flop.1.get(), destination));
                    }
                }
            } else if let Some(conjunction) = conjunctions.get(output) {
                let mut lows = conjunction.1.borrow_mut();
                if high {
                    lows.remove(input);
                } else {
                    lows.insert(input);
                }
                for destination in &conjunction.0 {
                    queue.push_back((output, !lows.is_empty(), destination));
                }
            } else if let Some(broadcaster) = broadcasters.get(output) {
                for destination in broadcaster {
                    queue.push_back((output, high, destination));
                }
            }
        }
    }
    lows * highs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            32000000,
            sum("broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
                .as_bytes())
        );
        assert_eq!(
            11687500,
            sum("broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
