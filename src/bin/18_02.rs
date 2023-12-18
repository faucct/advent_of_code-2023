#![feature(slice_group_by)]

use std::collections::BTreeSet;

fn sum(reader: impl std::io::BufRead) -> i64 {
    let instructions = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let tail = line.split_once("#").unwrap().1.strip_suffix(")").unwrap();
            let mut number = 0;
            for c in tail.chars() {
                number *= 16;
                number += c.to_digit(16).unwrap();
            }
            (['R', 'D', 'L', 'U'][number as usize % 16], number / 16)
        })
        .collect::<Vec<_>>();
    let mut height = 0..1;
    let mut row = 0;
    let mut width = 0..1;
    let mut column = 0;
    let mut edges = Vec::new();
    for &(instruction, length) in &instructions {
        match instruction {
            'R' => {
                edges.push((row, column, column + length as i64));
                column += length as i64;
            }
            'D' => row += length as i64,
            'L' => {
                edges.push((row, column - length as i64, column));
                column -= length as i64;
            }
            'U' => row -= length as i64,
            _ => panic!(),
        }
        height = height.start.min(row)..height.end.max(row + 1);
        width = width.start.min(column)..width.end.max(column + 1);
    }
    edges.sort();
    let mut row = BTreeSet::<i64>::new();
    let mut sum = 0;
    let mut prev = edges[0].0;
    for edges in edges.as_slice().group_by(|a, b| a.0 == b.0) {
        {
            let mut row = row.iter();
            while let Some(&start) = row.next() {
                let end = *row.next().unwrap();
                sum += (edges[0].0 - prev - 1) * (end - start + 1);
            }
        }
        prev = edges[0].0;
        let intervals = {
            let mut edges = edges.iter().map(|edge| (edge.1, edge.2)).peekable();
            let mut row = row.iter();
            let mut row =
                std::iter::from_fn(move || row.next().map(|start| (*start, *row.next().unwrap())))
                    .peekable();
            std::iter::from_fn(move || {
                if let Some(prev) = row.peek() {
                    if let Some(edge) = edges.peek() {
                        if edge.0 < prev.0 {
                            while let Some(prev) = row.peek_mut() {
                                prev.0 = prev.0.max(edge.1 + 1);
                                if prev.1 < prev.0 {
                                    row.next();
                                } else {
                                    break;
                                }
                            }
                            edges.next()
                        } else {
                            while let Some(edge) = edges.peek_mut() {
                                edge.0 = edge.0.max(prev.1 + 1);
                                if edge.1 < edge.0 {
                                    edges.next();
                                } else {
                                    break;
                                }
                            }
                            row.next()
                        }
                    } else {
                        row.next()
                    }
                } else {
                    edges.next()
                }
            })
        };
        for (start, end) in intervals {
            sum += end + 1 - start;
        }
        for edge in edges {
            if !row.remove(&edge.1) {
                row.insert(edge.1);
            }
            if !row.remove(&edge.2) {
                row.insert(edge.2);
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
            952408144115,
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
