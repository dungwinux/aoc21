fn valid_loc(loc: (i64, i64), lim: (usize, usize)) -> bool {
    loc.0 >= 0 && loc.0 < lim.0 as i64 && loc.1 >= 0 && loc.1 < lim.1 as i64
}

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut riskmap = vec![];
    for line in inp {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c as i64 - '0' as i64);
        }
        riskmap.push(row);
    }
    let mut state_store = vec![((0, 0), 0)];
    let lim = (riskmap.len(), riskmap[0].len());
    let dest = (lim.0 - 1, lim.1 - 1);

    let chain_direction: Vec<(i64, i64)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut min_map = vec![vec![i64::MAX; lim.1]; lim.0];
    while !state_store.is_empty() {
        let cur = state_store.pop().unwrap();
        let min = &mut min_map[cur.0 .0][cur.0 .1];
        if cur.1 < *min {
            *min = cur.1;
            if *min
                > min_map[dest.0][dest.1]
                    - (i64::abs(dest.0 as i64 - cur.0 .0 as i64)
                        + i64::abs(dest.1 as i64 - cur.0 .1 as i64)
                        - 1)
            {
                continue;
            }
            for dir in &chain_direction {
                let next_unsafe = (cur.0 .0 as i64 + dir.0, cur.0 .1 as i64 + dir.1);
                if valid_loc(next_unsafe, lim) {
                    let next = (next_unsafe.0 as usize, next_unsafe.1 as usize);
                    let val = riskmap[next.0][next.1];
                    state_store.push((next, cur.1 + riskmap[next.0][next.1]));
                }
            }
        }
    }
    Ok(min_map[dest.0][dest.1])
}
