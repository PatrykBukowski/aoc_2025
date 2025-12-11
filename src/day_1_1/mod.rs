use std::fs;
#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
    UNKNOWN,
}

struct _Rotation {
    direction: Direction,
    offset: u16,
}

pub fn start(path: &str) {
    let content = fs::read_to_string(path).expect("Should be able to read");

    let mut value = 50;
    let mut password = 0;

    for item in content.split("\n") {
        if item.is_empty() {
            continue;
        }
        let (dir, len) = item.split_at(1);
        let direction: Direction = match dir {
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => Direction::UNKNOWN,
        };
        let offset: i16 = match len.parse::<i16>() {
            Ok(value) => value,
            Err(_) => 0,
        };

        match direction {
            Direction::LEFT => {
                let pre_check = value - offset;
                let modulo = pre_check % 100;
                if modulo < 0 {
                    value = 100 + modulo;
                } else {
                    value = modulo;
                }
            }
            Direction::RIGHT => {
                let pre_check = value + offset;
                let modulo = pre_check % 100;
                value = modulo;
            }
            Direction::UNKNOWN => {}
        }
        if value == 100 {
            value = 0;
        }
        if value == 0 {
            password += 1;
        }
    }

    println!("DAY 1: {}", password);
}
