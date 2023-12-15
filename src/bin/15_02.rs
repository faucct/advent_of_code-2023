fn sum(reader: impl std::io::BufRead) -> usize {
    let mut boxes: [(std::collections::HashMap<String, usize>, Vec<usize>); 256] =
        std::array::from_fn(|_| Default::default());
    for line in reader.lines() {
        let line = line.unwrap();
        for step in line.split(",") {
            if let Some((label, value)) = step.split_once("=") {
                let mut hash = 0u8;
                for c in label.chars() {
                    hash = hash.wrapping_add(c as u8).wrapping_mul(17);
                }
                let the_box = &mut boxes[hash as usize];
                if let Some(entry) = the_box.0.get(label) {
                    the_box.1[*entry] = value.parse::<usize>().unwrap();
                } else {
                    the_box.0.insert(label.to_string(), the_box.1.len());
                    the_box.1.push(value.parse::<usize>().unwrap());
                }
            } else {
                let label = &step[..step.len() - 1];
                let mut hash = 0u8;
                for c in label.chars() {
                    hash = hash.wrapping_add(c as u8).wrapping_mul(17);
                }
                let the_box = &mut boxes[hash as usize];
                if let Some(entry) = the_box.0.remove(label) {
                    the_box.1[entry] = 0;
                }
            }
        }
    }
    let mut sum = 0;
    for (box_number, mut the_box) in boxes.into_iter().enumerate() {
        the_box.1.retain(|&i| i != 0);
        for (slot, focal_length) in the_box.1.into_iter().enumerate() {
            sum += (box_number + 1) * (slot + 1) * focal_length;
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
            145,
            sum("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
