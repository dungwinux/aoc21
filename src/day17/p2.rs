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

    let mut possible_hit = std::collections::HashSet::new();
    for dest_y in y_range.0..=y_range.1 {
        for step in 1..=x_range.1 * 2 {
            let s = dest_y + step * (step - 1) / 2;
            let y_vel = s / step;
            if y_vel * step != s {
                continue;
            }

            for x_vel in 0..=x_range.1 {
                let dest_x = if step <= x_vel {
                    x_vel * step - (step * (step - 1) / 2)
                } else {
                    x_vel * x_vel - (x_vel * (x_vel - 1) / 2)
                };
                // println!(
                //     "Step: {} Vel: ({} {}) {} {}",
                //     step, x_vel, y_vel, dest_x, dest_y
                // );
                if x_range.0 <= dest_x && dest_x <= x_range.1 {
                    // Add step to this for additional step tracking
                    possible_hit.insert((x_vel, y_vel));
                }
            }
        }
    }
    println!("{:?}", possible_hit);
    Ok(possible_hit.len() as i64)
}
