fn valid_loc(loc: (i64, i64), lim: (usize, usize)) -> bool {
    loc.0 >= 0 && loc.0 < lim.0 as i64 && loc.1 >= 0 && loc.1 < lim.1 as i64
}

fn real_to_virt(real: &(usize, usize), frame: &(usize, usize)) -> ((usize, usize), usize) {
    (
        (real.0 % frame.0, real.1 % frame.1),
        real.0 / frame.0 + real.1 / frame.1,
    )
}

fn val_real_to_virt(ori_map: &Vec<Vec<i64>>, real: &(usize, usize), frame: &(usize, usize)) -> i64 {
    let virt = real_to_virt(&real, &frame);
    let val = ori_map[virt.0 .0][virt.0 .1];
    (val + virt.1 as i64 - 1) % 9 + 1
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
    // Re-implement with BinaryHeap (PQ) for performance?
    let mut state_store = std::collections::VecDeque::from(vec![((0, 0), 0)]);
    let lim = (riskmap.len(), riskmap[0].len());
    let real_lim = (riskmap.len() * 5, riskmap[0].len() * 5);
    let dest = (real_lim.0 - 1, real_lim.1 - 1);

    // let mut heur_max: i64 = 0;
    // for y in 1..=dest.1 {
    //     heur_max += val_real_to_virt(&riskmap, (0, y), lim);
    // }
    // for x in 1..=dest.0 {
    //     heur_max += val_real_to_virt(&riskmap, (x, dest.1), lim);
    // }

    let chain_direction: Vec<(i64, i64)> = vec![(1, 0), (0, 1), (0, -1), (-1, 0)];
    let mut min_map = vec![vec![9 * (real_lim.1 + real_lim.0 - 1) as i64; real_lim.1]; real_lim.0];
    // min_map[dest.0][dest.1] = heur_max;
    while !state_store.is_empty() {
        let cur = state_store.pop_front().unwrap();
        let min = &mut min_map[cur.0 .0][cur.0 .1];
        if cur.1 < *min {
            *min = cur.1;
            if *min
                >= min_map[dest.0][dest.1]
                    - (dest.0 as i64 - cur.0 .0 as i64 + dest.1 as i64 - cur.0 .1 as i64)
            {
                continue;
            }
            for dir in &chain_direction {
                let next_unsafe = (cur.0 .0 as i64 + dir.0, cur.0 .1 as i64 + dir.1);
                if valid_loc(next_unsafe, real_lim) {
                    let next = (next_unsafe.0 as usize, next_unsafe.1 as usize);
                    // if next == dest {
                    //     println!("{:?} {:?}", next, virt_next);
                    // }
                    state_store.push_back((next, cur.1 + val_real_to_virt(&riskmap, &next, &lim)));
                }
            }
        }
    }
    Ok(min_map[dest.0][dest.1])
}
