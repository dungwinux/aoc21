use std::collections::HashSet;

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut opmode = false;
    let mut point_vec = HashSet::new();
    let mut opstack = vec![];
    for line in inp {
        if line == "" {
            opmode = true;
        } else if opmode {
            match line
                .strip_prefix("fold along ")
                .expect("Incorrect format")
                .split_once('=')
            {
                Some(("x", off)) => opstack.push((0, off.parse::<i64>()?)),
                Some(("y", off)) => opstack.push((1, off.parse::<i64>()?)),
                _ => (),
            }
        } else {
            if let Some((x, y)) = line.split_once(',') {
                point_vec.insert((x.parse::<i64>()?, y.parse::<i64>()?));
            }
        }
    }
    println!("{:?}", opstack);
    println!("{:?}", point_vec);
    for anchor in [opstack[0]] {
        let mut new_point = HashSet::new();
        for point in point_vec {
            match anchor.0 {
                1 => {
                    if point.1 > anchor.1 {
                        new_point.insert((point.0, point.1 - 2 * (point.1 - anchor.1)));
                    } else {
                        new_point.insert(point);
                    }
                }
                0 => {
                    if point.0 > anchor.1 {
                        new_point.insert((point.0 - 2 * (point.0 - anchor.1), point.1));
                    } else {
                        new_point.insert(point);
                    }
                }
                _ => (),
            };
        }
        point_vec = new_point;
    }
    println!("{:?}", point_vec);
    for i in 0..20 {
        for j in 0..20 {
            if point_vec.contains(&(j, i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    Ok(point_vec.len() as i64)
}
