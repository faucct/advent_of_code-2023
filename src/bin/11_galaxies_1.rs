fn sum(reader: impl std::io::BufRead) -> i32 {
    let image = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect::<Vec<_>>();
    let mut expansions = 0;
    let left_prefixes_expansions = (0..image[0].len())
        .map(|i| {
            if image.iter().all(|row| row[i] == '.' as u8) {
                expansions += 1;
            }
            expansions
        })
        .collect::<Vec<_>>();
    let mut expansions = 0;
    let top_prefixes_expansions = (0..image.len())
        .map(|i| {
            if image[i].iter().all(|&cell| cell == '.' as u8) {
                expansions += 1;
            }
            expansions
        })
        .collect::<Vec<_>>();
    let mut galaxies = Vec::new();
    for (&y_expansions, (y, row)) in top_prefixes_expansions.iter().zip(image.iter().enumerate()) {
        for (&x_expansions, (x, &cell)) in
            left_prefixes_expansions.iter().zip(row.iter().enumerate())
        {
            if cell == '#' as u8 {
                galaxies.push((y as i32 + y_expansions, x as i32 + x_expansions));
            }
        }
    }
    let mut sum = 0;
    for (suffix, &from) in galaxies.iter().enumerate() {
        for &to in &galaxies[suffix..] {
            sum += (from.0 - to.0).abs() + (from.1 - to.1).abs();
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
            374,
            sum("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
