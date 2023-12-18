fn sum(reader: impl std::io::BufRead) -> usize {
    let instructions = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (direction, tail) = line.split_once(" ").unwrap();
            (
                direction.chars().next().unwrap(),
                tail.split_once(" ").unwrap().0.parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut height = 0..1;
    let mut row = 0;
    let mut width = 0..1;
    let mut column = 0;
    for &(instruction, length) in &instructions {
        match instruction {
            'R' => column += length as i64,
            'D' => row += length as i64,
            'L' => column -= length as i64,
            'U' => row -= length as i64,
            _ => panic!(),
        }
        height = height.start.min(row)..height.end.max(row + 1);
        width = width.start.min(column)..width.end.max(column + 1);
    }
    let mut position = (-height.start as usize, -width.start as usize);
    let mut map =
        vec![vec![0; (width.end - width.start) as usize]; (height.end - height.start) as usize];
    for &(instruction, length) in &instructions {
        match instruction {
            'R' => {
                if map[position.0][position.1] == 0 {
                    map[position.0][position.1] = 2;
                }
                for _ in 0..length as usize {
                    position.1 += 1;
                    map[position.0][position.1] = 2;
                }
            }
            'D' => {
                for _ in 0..length as usize {
                    position.0 += 1;
                    map[position.0][position.1] = 1;
                }
            }
            'L' => {
                if map[position.0][position.1] == 0 {
                    map[position.0][position.1] = 2;
                }
                for _ in 0..length as usize {
                    position.1 -= 1;
                    map[position.0][position.1] = 2;
                }
            }
            'U' => {
                for _ in 0..length as usize {
                    map[position.0][position.1] = 1;
                    position.0 -= 1;
                }
            }
            _ => panic!(),
        }
    }
    // for row in &map {
    //     for cell in row {
    //         if *cell {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }
    // println!("");
    let mut sum = 0;
    for row in map {
        let mut inside = false;
        for cell in row {
            if inside || cell != 0 {
                print!("#");
                sum += 1;
            } else {
                print!(".");
            }
            inside ^= cell == 1;
        }
        println!("");
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            62,
            sum("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
