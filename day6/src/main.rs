struct LanternFish {
    timer: i32
}

fn lantern_count(initial: String, days: i32) -> usize {
    let mut fishes: Vec<LanternFish> = Vec::new();

    //Parse the input to create the initial fish
    let initial_fish = initial.split(",");
    for start in initial_fish {
        fishes.push(LanternFish{timer: start.parse().unwrap()});
    }

    for _ in 0..days {
        let mut new_fishes: Vec<LanternFish> = Vec::new();
        for fish in fishes.iter_mut() {
            if fish.timer == 0 {
                fish.timer = 6;
                new_fishes.push(LanternFish{timer: 8});
            } else {
                fish.timer -= 1;
            }
        }
        for fish in new_fishes {
            fishes.push(fish);
        }
    }

    return fishes.len();
}

fn day_count(initial: String, days: i32) -> i64 {
    let mut days_count:[i64; 9] = [0; 9];
    let initial_fish = initial.split(",");
    for start in initial_fish {
        days_count[start.parse::<usize>().unwrap()] += 1
    }

    // println!("{:?}", days_count);
    for _ in 0..days {
        let mut next:[i64; 9] = [0; 9];
        next[6] = days_count[0];
        next[8] = days_count[0];
        for i in 0..8 {
            next[i] += days_count[i+1]
        }
        days_count = next;
        // println!("{:?}", days_count)
    }
    let mut total:i64 = 0;
    for count in days_count {
        total += count;
    }
    return total;
}

fn main() {
    let input = String::from("1,1,3,1,3,2,1,3,1,1,3,1,1,2,1,3,1,1,3,5,1,1,1,3,1,2,1,1,1,1,4,4,1,2,1,2,1,1,1,5,3,2,1,5,2,5,3,3,2,2,5,4,1,1,4,4,1,1,1,1,1,1,5,1,2,4,3,2,2,2,2,1,4,1,1,5,1,3,4,4,1,1,3,3,5,5,3,1,3,3,3,1,4,2,2,1,3,4,1,4,3,3,2,3,1,1,1,5,3,1,4,2,2,3,1,3,1,2,3,3,1,4,2,2,4,1,3,1,1,1,1,1,2,1,3,3,1,2,1,1,3,4,1,1,1,1,5,1,1,5,1,1,1,4,1,5,3,1,1,3,2,1,1,3,1,1,1,5,4,3,3,5,1,3,4,3,3,1,4,4,1,2,1,1,2,1,1,1,2,1,1,1,1,1,5,1,1,2,1,5,2,1,1,2,3,2,3,1,3,1,1,1,5,1,1,2,1,1,1,1,3,4,5,3,1,4,1,1,4,1,4,1,1,1,4,5,1,1,1,4,1,3,2,2,1,1,2,3,1,4,3,5,1,5,1,1,4,5,5,1,1,3,3,1,1,1,1,5,5,3,3,2,4,1,1,1,1,1,5,1,1,2,5,5,4,2,4,4,1,1,3,3,1,5,1,1,1,1,1,1");
    // println!("Part 1: {}", lantern_count(input, 80));
    println!("Part 2: {}", day_count(input, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lantern_count() {
        let initial = String::from("3,4,3,1,2");
        assert_eq!(lantern_count(initial, 80), 5934);
    }

    #[test]
    fn test_day_count() {
        let initial = String::from("3,4,3,1,2");
        assert_eq!(day_count(initial, 256), 26984457539);
    }
}
