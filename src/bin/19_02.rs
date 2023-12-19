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
    const RANGE: std::ops::RangeInclusive<u64> = 1..=4000;
    let mut sum = 0;
    let mut queue = vec![("in", [RANGE; 4])];
    while let Some((workflow, mut categories)) = queue.pop() {
        match workflow {
            "A" => {
                sum += categories
                    .into_iter()
                    .map(|category| category.end() + 1 - category.start())
                    .product::<u64>()
            }
            "R" => {}
            workflow => {
                let workflow = &workflows[workflow];
                for (rule, then) in workflow {
                    match *rule {
                        Rule::Greater(category, greater) => {
                            let values = categories[category].clone();
                            if *values.end() <= greater {
                                continue;
                            }
                            if greater < *values.start() {
                                queue.push((then.as_str(), categories));
                                break;
                            } else {
                                let mut then_categories = categories.clone();
                                then_categories[category] = greater + 1..=*values.end();
                                categories[category] = *values.start()..=greater;
                                queue.push((then.as_str(), then_categories));
                            }
                        }
                        Rule::Lesser(category, lesser) => {
                            let values = categories[category].clone();
                            if lesser <= *values.start() {
                                continue;
                            }
                            if *values.end() < lesser {
                                queue.push((then.as_str(), categories));
                                break;
                            } else {
                                let mut then_categories = categories.clone();
                                then_categories[category] = *values.start()..=lesser - 1;
                                categories[category] = lesser..=*values.end();
                                queue.push((then.as_str(), then_categories));
                            }
                        }
                        Rule::Else => {
                            queue.push((then.as_str(), categories));
                            break;
                        }
                    }
                }
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
            167409079868000,
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
