pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let (raw_x, raw_y) = inp[0]
        .strip_prefix("target area: ")
        .expect("Wrong format")
        .split_once(", ")
        .expect("Wrong format");
    let raw_x_range = raw_x
        .strip_prefix("x=")
        .expect("Wrong x define")
        .split_once("..")
        .expect("Wrong x range");
    let x_range = (raw_x_range.0.parse::<i64>()?, raw_x_range.1.parse::<i64>()?);
    let raw_y_range = raw_y
        .strip_prefix("y=")
        .expect("Wrong y define")
        .split_once("..")
        .expect("Wrong y range");
    let y_range = (raw_y_range.0.parse::<i64>()?, raw_y_range.1.parse::<i64>()?);

    println!("{:?} {:?}", x_range, y_range);

    let mut possible_hit = vec![];
    for dest_y in y_range.0..=y_range.1 {
        for step in 1..=x_range.1 * 2 {
            let s = dest_y + step * (step - 1) / 2;
            let y_vel = s / step;
            if y_vel * step != s {
                continue;
            }

            for x_vel in 0..step {
                let dest_x = x_vel * x_vel - (x_vel * (x_vel - 1) / 2);
                if x_range.0 <= dest_x && dest_x <= x_range.1 {
                    possible_hit.push((x_vel, y_vel, step));
                }
            }
        }
    }
    println!("{:?}", possible_hit);
    let mut best_y = 0;
    for init_vel in possible_hit.into_iter() {
        let mut dy = init_vel.1;
        let mut cur_y = 0;
        for _ in 1..init_vel.2 {
            // let cur_x = if init_vel.0 < step {
            //     init_vel.0 * step - (step * (step - 1) / 2)
            // } else {
            //     init_vel.0 * init_vel.0 - (init_vel.0 * (init_vel.0 - 1) / 2)
            // };
            cur_y += dy;
            dy -= 1;
            if cur_y > best_y {
                best_y = cur_y;
            }
            if dy == 0 {
                break;
            }
        }
    }
    Ok(best_y)
}
