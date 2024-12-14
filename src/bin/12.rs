// use advent_of_code::Point;
// use hashbrown::{HashMap, HashSet};
// use itertools::Itertools;

advent_of_code::solution!(12);

// fn flood_fill(
//     points_map: &HashMap<Point, char>,
//     start: Point,
//     value: char,
//     visited: &mut HashSet<Point>,
//     cluster: &mut HashSet<Point>,
// ) {
//     // If the point is out of bounds or already visited, return
//     if visited.contains(&start) {
//         return;
//     }

//     // Mark this point as visited
//     visited.insert(start);

//     // If the point has the correct value, add it to the current cluster
//     if let Some(&point_value) = points_map.get(&start) {
//         if point_value == value {
//             cluster.insert(start);
//         }
//     } else {
//         return;
//     }

//     let mut directions: Vec<Point> = vec![
//         Point::new(start.x, start.y + 1),
//         Point::new(start.x + 1, start.y),
//     ];

//     if start.x > 0 {
//         directions.push(Point::new(start.x - 1, start.y));
//     }

//     if start.y > 0 {
//         directions.push(Point::new(start.x, start.y - 1));
//     }

//     // Explore all 4 directions from the current point
//     for &dir in &directions {
//         flood_fill(points_map, dir, value, visited, cluster);
//     }
// }

// fn find_contiguous_clusters(points_map: &HashMap<Point, char>) -> Vec<(char, HashSet<Point>)> {
//     let mut visited = HashSet::new();
//     let mut clusters = Vec::new();

//     // Iterate through all the points in the map
//     for &point in points_map.keys() {
//         if !visited.contains(&point) {
//             if let Some(&value) = points_map.get(&point) {
//                 // Start a flood fill if the point hasn't been visited
//                 let mut cluster = HashSet::new();
//                 flood_fill(points_map, point, value, &mut visited, &mut cluster);
//                 clusters.push((value, cluster));
//             }
//         }
//     }

//     clusters
// }

pub fn part_one(_input: &str) -> Option<usize> {
    // let mut map: HashMap<Point, char> = HashMap::new();
    // for (y, line) in input.lines().enumerate() {
    //     for (x, c) in line.chars().enumerate() {
    //         map.insert(Point::new(x, y), c);
    //     }
    // }
    // let clusters = find_contiguous_clusters(&map);
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
