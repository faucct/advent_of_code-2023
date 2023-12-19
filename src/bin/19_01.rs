enum Rule {
    Greater(usize, u64),
    Lesser(usize, u64),
    Else,
}

fn category(string: &str) -> usize {
    match string {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!(),
    }
}

fn sum(reader: impl std::io::BufRead) -> u64 {
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut workflows = std::collections::HashMap::<String, Vec<(Rule, String)>>::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (name, line) = line.strip_suffix("}").unwrap().split_once("{").unwrap();
        workflows.insert(
            name.to_string(),
            line.split(",")
                .map(|rule| {
                    if let Some((case, then)) = rule.split_once(":") {
                        (
                            if let Some((c, size)) = case.split_once(">") {
                                Rule::Greater(category(c), size.parse().unwrap())
                            } else if let Some((c, size)) = case.split_once("<") {
                                Rule::Lesser(category(c), size.parse().unwrap())
                            } else {
                                panic!();
                            },
                            then.to_string(),
                        )
                    } else {
                        (Rule::Else, rule.to_string())
                    }
                })
                .collect(),
        );
    }
    lines
        .map(|part| {
            let mut categories = [0u64; 4];
            for category_value in part
                .strip_prefix("{")
                .unwrap()
                .strip_suffix("}")
                .unwrap()
                .split(",")
            {
                let category_value = category_value.split_once("=").unwrap();
                categories[category(category_value.0)] = category_value.1.parse().unwrap();
            }
            let mut workflow = &workflows["in"];
            loop {
                for rule in workflow {
                    match rule.0 {
                        Rule::Greater(category, value) => {
                            if categories[category] <= value {
                                continue;
                            }
                        }
                        Rule::Lesser(category, value) => {
                            if categories[category] >= value {
                                continue;
                            }
                        }
                        Rule::Else => {}
                    }
                    match rule.1.as_str() {
                        "A" => return categories.into_iter().sum(),
                        "R" => return 0,
                        then => {
                            workflow = &workflows[then];
                            break;
                        }
                    }
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            19114,
            sum("px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
