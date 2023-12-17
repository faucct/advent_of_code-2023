fn id(game: &str) -> usize {
    let (id, game) = game
        .strip_prefix("Game ")
        .unwrap()
        .split_once(": ")
        .unwrap();
    let id: usize = id.parse().unwrap();
    for set in game.split("; ") {
        let mut colors_count = [12, 13, 14];
        for colored in set.split(", ") {
            let (count, color) = colored.split_once(' ').unwrap();
            let count: usize = count.parse().unwrap();
            let remaining = &mut colors_count[["red", "green", "blue"]
                .into_iter()
                .position(|c| c == color)
                .unwrap()];
            if *remaining < count {
                return 0;
            }
            *remaining -= count;
        }
    }
    id
}

fn sum(reader: impl std::io::BufRead) -> usize {
    reader
        .lines()
        .map(|game| {
            let game = game.unwrap();
            if !game.is_empty() {
                id(&game)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            1,
            id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            2,
            id("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            0,
            id("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            0,
            id("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")
        );
    }

    #[test]
    fn example5() {
        assert_eq!(
            5,
            id("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
        );
    }

    #[test]
    fn example() {
        assert_eq!(
            8,
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
