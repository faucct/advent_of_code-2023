fn sum(reader: impl std::io::BufRead) -> usize {
    fn rec(
        mut chars: impl Clone + Iterator<Item = char>,
        group: usize,
        mut groups: impl Clone + Iterator<Item = usize>,
    ) -> usize {
        if let Some(c) = chars.next() {
            match c {
                '.' => {
                    if group != 0 && groups.next() != Some(group) {
                        0
                    } else {
                        rec(chars, 0, groups)
                    }
                }
                '#' => rec(chars, group + 1, groups),
                '?' => {
                    rec(chars.clone(), group + 1, groups.clone())
                        + if group != 0 && groups.next() != Some(group) {
                            0
                        } else {
                            rec(chars, 0, groups)
                        }
                }
                c => panic!("{:?}", c),
            }
        } else {
            if groups.next() == (if group != 0 { Some(group) } else { None })
                && groups.next() == None
            {
                1
            } else {
                0
            }
        }
    }
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (records, groups) = line.split_once(" ").unwrap();
        sum += rec(
            records.chars(),
            0,
            groups.split(",").map(|group| group.parse().unwrap()),
        );
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            21,
            sum("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
