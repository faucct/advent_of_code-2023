fn sum(reader: impl std::io::BufRead) -> usize {
    let mut bricks = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut brick = line
                .split("~")
                .flat_map(|end| end.split(",").map(|i| i.parse::<usize>().unwrap()));
            let brick: [usize; 6] = std::array::from_fn(|_| brick.next().unwrap());
            (
                brick[0].min(brick[3])..=brick[0].max(brick[3]),
                brick[1].min(brick[4])..=brick[1].max(brick[4]),
                brick[2].min(brick[5])..=brick[2].max(brick[5]),
                true,
            )
        })
        .collect::<Vec<_>>();
    let mut width = (0, 0);
    for brick in &bricks {
        width = width.max((*brick.0.end() + 1, *brick.1.end() + 1));
    }
    let mut bottoms = vec![vec![usize::MAX; width.1]; width.0];
    bricks.sort_unstable_by_key(|brick| *brick.2.start());
    let mut disintegratable = bricks.len();
    for id in 0..bricks.len() {
        let mut z = 0;
        let mut top_bottom_brick = usize::MAX;
        let brick = bricks[id].clone();
        for i in brick.0 {
            for j in brick.1.clone() {
                let bottom_brick = std::mem::replace(&mut bottoms[i][j], id);
                if bottom_brick != usize::MAX {
                    if z != 0 {
                        if bottom_brick != top_bottom_brick {
                            if z == bricks[bottom_brick].2.end() + 1 {
                                top_bottom_brick = usize::MAX;
                            } else if z < bricks[bottom_brick].2.end() + 1 {
                                z = *bricks[bottom_brick].2.end() + 1;
                                top_bottom_brick = bottom_brick;
                            }
                        }
                    } else {
                        z = *bricks[bottom_brick].2.end() + 1;
                        top_bottom_brick = bottom_brick;
                    }
                }
            }
        }
        bricks[id].2 = z..=z + brick.2.end() - brick.2.start();
        if top_bottom_brick != usize::MAX
            && std::mem::replace(&mut bricks[top_bottom_brick].3, false)
        {
            disintegratable -= 1;
        }
    }
    disintegratable
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            5,
            sum("1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}
