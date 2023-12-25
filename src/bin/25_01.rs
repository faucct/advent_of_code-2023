fn sum(reader: impl std::io::BufRead) -> usize {
    let mut graph = std::collections::HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let (from, to) = line.split_once(": ").unwrap();
        graph.entry(from.to_string()).or_insert(std::collections::HashSet::new());
        for to in to.split(" ") {
            graph.get_mut(from).unwrap().insert(to.to_string());
            graph.entry(to.to_string()).or_default().insert(from.to_string());
        }
    }
    let source = graph.keys().next().unwrap();
    graph.keys().map(|sink| {
        if sink == source {
            return 0;
        }
        let mut flow = std::collections::HashSet::with_capacity(graph.len());
        fn rec<'b, 'a : 'b>(
            graph: &'a std::collections::HashMap<String, std::collections::HashSet<String>>,
            flow: &std::collections::HashSet<(&'b String, &'b String)>,
            sink: &'b String,
            visited: &mut std::collections::HashSet<&'b String>,
            path: &mut Vec<&'b String>,
            from: &'b String,
        ) -> bool {
            if from == sink {
                return true;
            }
            for to in &graph[from] {
                if !flow.contains(&(from, to)) && visited.insert(to) {
                    path.push(to);
                    if rec(graph, flow, sink, visited, path, to) {
                        return true;
                    }
                    path.pop();
                }
            }
            false
        }
        for _ in 0..3 {
            let mut path = Vec::new();
            let mut visited = std::collections::HashSet::new();
            visited.insert(source);
            if !rec(&graph, &flow, sink, &mut visited, &mut path, source) {
                panic!("\n{graph:?}\n{flow:?}\n{visited:?}");
            }
            let mut prev = source;
            for next in path {
                if !flow.remove(&(next, prev)) {
                    flow.insert((prev, next));
                }
                prev = next;
            }
        }
        let mut path = Vec::new();
        let mut visited = std::collections::HashSet::new();
        visited.insert(source);
        if rec(&graph, &flow, sink, &mut visited, &mut path, source) {
            return 0;
        }
        visited.len() * (graph.len() - visited.len())
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            54,
            sum("jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
