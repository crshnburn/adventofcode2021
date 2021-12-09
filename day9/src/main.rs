use std::collections::HashSet;

const INPUT: &str = include_str!("input");

fn main() {
    let (part1, part2) = find_low_points(INPUT);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn find_low_points(input: &str) -> (i32, i32) {
    let mut risk = 0;
    let mut basin_sizes: Vec<i32> = Vec::new();

    let map: Vec<Vec<i32>> = input.lines().collect::<Vec<&str>>().iter()
                                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();

    // println!("{:?}", map);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let val = map[y][x];
            let mut adjacent: Vec<i32> = Vec::new();
            if y > 0 {
                adjacent.push(map[y-1][x]);
            }
            if y < map.len()-1 {
                adjacent.push(map[y+1][x]);
            }
            if x > 0 {
                adjacent.push(map[y][x-1]);
            }
            if x < map[y].len()-1 {
                adjacent.push(map[y][x+1]);
            }

            // println!("{:?}", adjacent);

            if adjacent.iter().filter(|p| p<=&&val).collect::<Vec<&i32>>().len() == 0 {
                // println!("{:?} {}", adjacent, val);
                risk += 1 + val;
                basin_sizes.push(calc_size(&map, y, x));
            }
        }
    }

    // println!("{:?}", basin_sizes);
    basin_sizes.sort();

    return (risk, basin_sizes[basin_sizes.len()-1]*basin_sizes[basin_sizes.len()-2]*basin_sizes[basin_sizes.len()-3]);
}

fn calc_size(map: &Vec<Vec<i32>>, y: usize, x: usize) -> i32 {
    let mut size = 0;
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    stack.push((y, x));
    while stack.len() > 0 {
        let (j, i) = stack.pop().unwrap();
        if !visited.contains(&(j, i)) && map[j][i] != 9 {
            size += 1;
            visited.insert((j,i));
            let adjacent = get_adjacent(map, j, i);
            for pos in adjacent {
                stack.push(pos);
            }
        }
    }



    return size;
}

fn get_adjacent(map: &Vec<Vec<i32>>, y: usize, x: usize) -> Vec<(usize, usize)> {
    let mut adjacent: Vec<(usize, usize)> = Vec::new();

    if y > 0 {
        adjacent.push((y-1,x));
    }
    if y < map.len()-1 {
        adjacent.push((y+1,x));
    }
    if x > 0 {
        adjacent.push((y,x-1));
    }
    if x < map[y].len()-1 {
        adjacent.push((y,x+1));
    }

    return adjacent;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_low_points() {
        let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
        assert_eq!(find_low_points(input), (15,1134));
    }
}
