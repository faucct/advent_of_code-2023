fn sum(reader: impl std::io::BufRead) -> usize {
    fn rec(
        cache: &mut std::collections::HashMap<(usize, usize), usize>,
        mut chars: impl Clone + Iterator<Item = char>,
        group: usize,
        mut groups: impl Clone + Iterator<Item = usize>,
    ) -> usize {
        let key = (chars.size_hint().0, groups.size_hint().0);
        if group == 0 {
            if let Some(cached) = cache.get(&key) {
                return *cached;
            }
        }
        let result = if let Some(c) = chars.next() {
            match c {
                '.' => {
                    if group != 0 && groups.next() != Some(group) {
                        0
                    } else {
                        rec(cache, chars, 0, groups)
                    }
                }
                '#' => rec(cache, chars, group + 1, groups),
                '?' => {
                    rec(cache, chars.clone(), group + 1, groups.clone())
                        + if group != 0 && groups.next() != Some(group) {
                            0
                        } else {
                            rec(cache, chars, 0, groups)
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
        };
        if group == 0 {
            cache.insert(key, result);
        }
        result
    }
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (records, groups) = line.split_once(" ").unwrap();
        let records = (0..5)
            .flat_map(|_| std::iter::once('?').chain(records.chars()))
            .skip(1)
            .collect::<Vec<_>>();
        let groups = (0..5)
            .flat_map(|_| groups.split(",").map(|group| group.parse().unwrap()))
            .collect::<Vec<_>>();
        sum += rec(
            &mut Default::default(),
            records.iter().copied(),
            0,
            groups.iter().copied(),
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
            525152,
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
