fn parse_data(s: &str) -> (usize, usize) {
    let (a, b) = s.split_once(',').expect("Could not parse line of vents");
    (
        a.parse::<usize>().expect("vent is not number"),
        b.parse::<usize>().expect("vent is not number"),
    )
}

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let sz = 1000;
    let mut board = vec![vec![0usize; sz]; sz];
    for raw_data in inp {
        let vent_info: Vec<&str> = raw_data.split_whitespace().filter(|&s| s != "->").collect();
        println!("{:?}", vent_info);
        let beg = parse_data(vent_info[0]);
        let end = parse_data(vent_info[1]);
        if beg.0 == end.0 {
            let a = usize::min(beg.1, end.1);
            let b = usize::max(beg.1, end.1);
            for i in a..=b {
                board[beg.0][i] += 1;
            }
        } else if beg.1 == end.1 {
            let a = usize::min(beg.0, end.0);
            let b = usize::max(beg.0, end.0);
            for i in a..=b {
                board[i][beg.1] += 1;
            }
        } else {
            let x: i64 = end.0 as i64 - beg.0 as i64;
            let y: i64 = end.1 as i64 - beg.1 as i64;
            let dx: i64 = if x < 0 { -1 } else { 1 };
            let dy: i64 = if y < 0 { -1 } else { 1 };
            for i in 0..=i64::abs(x) {
                board[(beg.0 as i64 + i * dx) as usize][(beg.1 as i64 + i * dy) as usize] += 1;
            }
        }
    }
    // println!("{:?}", board);
    let mut counter = vec![0u64; sz];
    for r in board {
        for i in r {
            counter[i] += 1;
        }
    }

    Ok(counter.into_iter().skip(2).sum::<u64>() as i64)
}
