use std::collections::HashMap;

use advent_of_code::Point;

advent_of_code::solution!(4);

// this one sucks

fn build_points(input: &str) -> (HashMap<Point, char>, usize, usize) {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    let mut map: HashMap<Point, char> = HashMap::new();
    for (y, s) in input.lines().enumerate() {
        for (x, c) in s.chars().enumerate() {
            map.insert(Point::new(x, y), c);
            if x > max_x {
                max_x = x;
            }
        }
        if y > max_y {
            max_y = y;
        }
    }
    (map, max_x, max_y)
}

fn find_xmas(points: HashMap<Point, char>, max_x: usize, max_y: usize) -> u32 {
    //let comparison = vec!['X','M','A','S'];
    let mut count = 0;
    for k in points.keys() {
        // left
        if k.x > 2 {
            if points[&Point::new(k.x, k.y)] == 'X'
                && points[&Point::new(k.x - 1, k.y)] == 'M'
                && points[&Point::new(k.x - 2, k.y)] == 'A'
                && points[&Point::new(k.x - 3, k.y)] == 'S'
            {
                count += 1;
            }

            // diag up-left
            if k.y > 2
                && points[&Point::new(k.x, k.y)] == 'X'
                && points[&Point::new(k.x - 1, k.y - 1)] == 'M'
                && points[&Point::new(k.x - 2, k.y - 2)] == 'A'
                && points[&Point::new(k.x - 3, k.y - 3)] == 'S'
            {
                count += 1;
            }

            if k.y < (max_y - 2) {
                // diag down-left
                if points[&Point::new(k.x, k.y)] == 'X'
                    && points[&Point::new(k.x - 1, k.y + 1)] == 'M'
                    && points[&Point::new(k.x - 2, k.y + 2)] == 'A'
                    && points[&Point::new(k.x - 3, k.y + 3)] == 'S'
                {
                    count += 1;
                }
            }
        }

        if k.x < (max_x - 2) {
            if points[&Point::new(k.x, k.y)] == 'X'
                && points[&Point::new(k.x + 1, k.y)] == 'M'
                && points[&Point::new(k.x + 2, k.y)] == 'A'
                && points[&Point::new(k.x + 3, k.y)] == 'S'
            {
                count += 1;
            }

            // diag up-right
            if k.y > 2
                && points[&Point::new(k.x, k.y)] == 'X'
                && points[&Point::new(k.x + 1, k.y - 1)] == 'M'
                && points[&Point::new(k.x + 2, k.y - 2)] == 'A'
                && points[&Point::new(k.x + 3, k.y - 3)] == 'S'
            {
                count += 1;
            }

            if k.y < (max_y - 2) {
                // diag down-right
                if points[&Point::new(k.x, k.y)] == 'X'
                    && points[&Point::new(k.x + 1, k.y + 1)] == 'M'
                    && points[&Point::new(k.x + 2, k.y + 2)] == 'A'
                    && points[&Point::new(k.x + 3, k.y + 3)] == 'S'
                {
                    count += 1;
                }
            }
        }

        // up
        if k.y > 2
            && points[&Point::new(k.x, k.y)] == 'X'
            && points[&Point::new(k.x, k.y - 1)] == 'M'
            && points[&Point::new(k.x, k.y - 2)] == 'A'
            && points[&Point::new(k.x, k.y - 3)] == 'S'
        {
            count += 1;
        }

        // down
        if k.y < (max_y - 2)
            && points[&Point::new(k.x, k.y)] == 'X'
            && points[&Point::new(k.x, k.y + 1)] == 'M'
            && points[&Point::new(k.x, k.y + 2)] == 'A'
            && points[&Point::new(k.x, k.y + 3)] == 'S'
        {
            count += 1;
        }
    }
    count
}

fn is_x_max(upleft: char, upright: char, downleft: char, downright: char) -> bool {
    (upleft == 'M' && upright == 'M' && downleft == 'S' && downright == 'S')
        || (upleft == 'S' && upright == 'S' && downleft == 'M' && downright == 'M')
        || (upleft == 'M' && upright == 'S' && downleft == 'M' && downright == 'S')
        || (upleft == 'S' && upright == 'M' && downleft == 'S' && downright == 'M')
}

fn find_xmas_v2(points: HashMap<Point, char>, max_x: usize, max_y: usize) -> u32 {
    let mut count: u32 = 0;
    for k in points.keys() {
        let has_corners = k.x > 0 && k.y > 0 && k.x < max_x && k.y < max_y;
        let is_a = points[k] == 'A';

        if is_a && has_corners {
            //println!("Testing A at point: {}", k);
            let up_left = Point::new(k.x - 1, k.y - 1);
            let up_right = Point::new(k.x + 1, k.y - 1);
            let down_lft = Point::new(k.x - 1, k.y + 1);
            let down_right = Point::new(k.x + 1, k.y + 1);

            let up_left_char = points[&up_left];
            let up_right_char = points[&up_right];
            let down_left_char = points[&down_lft];
            let down_right_char = points[&down_right];

            if is_x_max(up_left_char, up_right_char, down_left_char, down_right_char) {
                //println!("XMAS A: {}", k);
                count += 1
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let (points, max_x, max_y) = build_points(input);
    let count = find_xmas(points, max_x, max_y);
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (points, max_x, max_y) = build_points(input);
    let count = find_xmas_v2(points, max_x, max_y);
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
