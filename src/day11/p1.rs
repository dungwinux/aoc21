fn valid_loc(loc: (i64, i64), lim: (i64, i64)) -> bool {
    loc.0 >= 0 && loc.0 < lim.0 && loc.1 >= 0 && loc.1 < lim.1
}

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut oct_map = vec![];
    for line in inp {
        let oct_row: Vec<i64> = line.chars().map(|x| x as i64 - '0' as i64).collect();
        oct_map.push(oct_row);
    }
    let chain_direction: Vec<(i64, i64)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    let mut flash_count = 0;
    let mut step = 100;
    while step > 0 {
        step -= 1;
        println!("{:?}", oct_map);
        let mut stack = vec![];
        let mut iter_i = 0;
        for row in oct_map.iter_mut() {
            let mut iter_j = 0;
            for oct in row.iter_mut() {
                *oct += 1;
                if *oct > 9 {
                    stack.push((iter_i, iter_j));
                }
                iter_j += 1;
            }
            iter_i += 1;
        }
        let sz = (oct_map.len() as i64, oct_map[0].len() as i64);
        let mut covered = vec![];
        while !stack.is_empty() {
            let loc = stack.pop().unwrap();
            if covered.iter().any(|x| *x == loc) {
                continue;
            }
            covered.push(loc);
            for dir in &chain_direction {
                let new_unsafe_loc = (loc.0 as i64 + (*dir).0, loc.1 as i64 + (*dir).1);
                if valid_loc(new_unsafe_loc, sz) {
                    let new_loc = (new_unsafe_loc.0 as usize, new_unsafe_loc.1 as usize);
                    if !covered.iter().any(|x| *x == new_loc) {
                        oct_map[new_loc.0][new_loc.1] += 1;
                        if oct_map[new_loc.0][new_loc.1] > 9 {
                            stack.push(new_loc);
                        }
                    }
                }
            }
            oct_map[loc.0][loc.1] = 0;
        }
        flash_count += covered.len() as i64;
    }
    Ok(flash_count)
}
