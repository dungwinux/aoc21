struct Loc {
    depth: i32,
    line: i32,
}

pub fn solve(inp: Vec<&str>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut my_loc = Loc { depth: 0, line: 0 };
    for raw_cmd in inp {
        let (x, y) = raw_cmd.split_once(' ').expect("Wrong command");
        println!("{} {}", x, y);
        let step = y.parse::<i32>()?;
        match x {
            "forward" => my_loc.line += step,
            "down" => my_loc.depth += step,
            "up" => my_loc.depth -= step,
            &_ => (),
        }
    }
    Ok(my_loc.depth * my_loc.line)
}
