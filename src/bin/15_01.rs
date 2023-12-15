fn sum(reader: impl std::io::BufRead) -> usize {
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        for step in line.split(",") {
            let mut hash = 0u8;
            for c in step.chars() {
                hash = hash.wrapping_add(c as u8).wrapping_mul(17);
            }
            sum += hash as usize;
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
            1320,
            sum("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
