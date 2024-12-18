advent_of_code::solution!(18);

use std::usize;

use advent_of_code::Point;
use pathfinding::prelude::*;

fn get_neighbors(p: (usize, usize), grid: &Grid) -> Vec<((usize, usize), usize)> {
    let neigh = grid.neighbours(p);
    neigh.iter().map(|x| ((x.0, x.1), 1)).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let coords: Vec<Point> = input
        .lines()
        .take(1024)
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Point::new(x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect();
    let mut grid = Grid::new(71, 71);
    grid.add_borders();
    grid.disable_diagonal_mode();
    grid.fill();
    for point in coords {
        if !grid.remove_vertex((point.x, point.y)) {
            panic!("oh no")
        }
    }
    let (_, weight) = dijkstra(
        &(0usize, 0usize),
        |&(x, y)| get_neighbors((x, y), &grid),
        |p| p.0 == 70 && p.1 == 70,
    )
    .unwrap();
    Some(weight)
}

pub fn part_two(input: &str) -> Option<String> {
    let coords: Vec<Point> = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Point::new(x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect();
    let mut grid = Grid::new(71, 71);
    grid.add_borders();
    grid.disable_diagonal_mode();
    grid.fill();
    for (ix, point) in coords.iter().enumerate() {
        grid.remove_vertex((point.x, point.y));
        if ix > 1023
            && dijkstra(
                &(0usize, 0usize),
                |&(x, y)| get_neighbors((x, y), &grid),
                |p| p.0 == 70 && p.1 == 70,
            )
            .is_none()
        {
            let output = format!("{},{}", point.x, point.y);
            return Some(output);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    //use super::*;

    // test relies on different grid sizes not in input etc.

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(22));
    // }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
