fn sum(reader: impl std::io::BufRead) -> usize {
    reader.lines().map(|game| {
        let game = game.unwrap();
        if !game.is_empty() {
            let mut max_colors_count: [usize; 3] = [0; 3];
            let (_id, game) = game.strip_prefix("Game ").unwrap().split_once(": ").unwrap();
            for set in game.split("; ") {
                let mut colors_count = [0; 3];
                for colored in set.split(", ") {
                    let (count, color) = colored.split_once(' ').unwrap();
                    let count: usize = count.parse().unwrap();
                    let color = ["red", "green", "blue"].into_iter().position(|c| c == color).unwrap();
                    colors_count[color] += count;
                }
                for (color, count) in colors_count.into_iter().enumerate() {
                    max_colors_count[color] = max_colors_count[color].max(count);
                }
            }
            max_colors_count.into_iter().product()
        } else {
            0
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            2286,
            sum("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
