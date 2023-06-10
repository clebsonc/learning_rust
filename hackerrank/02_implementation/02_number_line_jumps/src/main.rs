// https://www.hackerrank.com/challenges/kangaroo/

use ::std::io;

struct Kangaroo {
    position: i32,
    speed: i32,
}

/// Assumes K1 is always ahead K2.
fn location_at_same_time(ahead: Kangaroo, behind: Kangaroo) -> bool {
    if ahead.speed >= behind.speed {
        return false;
    }

    let k_distance = ahead.position - behind.position;
    // the speed of the second kangaroo is always smaller, so just compute
    // the difference of speed of the first by the second.
    let k_speed_difference = behind.speed - ahead.speed;

    // the proportion of distance regarding the difference should have a module equals 0:
    if k_distance % k_speed_difference != 0 {
        return false;
    }

    true
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let values: Vec<&str> = buffer.trim().split(' ').collect();
    let values: Vec<i32> = values.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let behind = Kangaroo {
        position: values[0],
        speed: values[1],
    };
    let ahead = Kangaroo {
        position: values[2],
        speed: values[3],
    };
    let result = location_at_same_time(ahead, behind);
    match result {
        true => println!("Yes"),
        false => println!("No"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let k1 = Kangaroo {
            position: 2,
            speed: 1,
        };
        let k2 = Kangaroo {
            position: 1,
            speed: 2,
        };
        let result = location_at_same_time(k1, k2);
        assert!(result);
    }
    #[test]
    fn test2() {
        let k1 = Kangaroo {
            position: 0,
            speed: 3,
        };
        let k2 = Kangaroo {
            position: 4,
            speed: 2,
        };
        let result = location_at_same_time(k2, k1);
        assert!(result);
    }
    #[test]
    fn test3() {
        let behind = Kangaroo {
            position: 0,
            speed: 2,
        };
        let ahead = Kangaroo {
            position: 5,
            speed: 3,
        };
        let result = location_at_same_time(ahead, behind);
        assert!(!result);
    }
}
