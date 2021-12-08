fn has_feat(s: &str, feat: &str) -> bool {
    let mut count = 0;
    for ch in s.chars() {
        if let Some(_) = feat.find(ch) {
            count += 1;
        }
    }
    count == feat.len()
}

fn rem_feat(s: &str, feat: &str) -> String {
    s.clone()
        .chars()
        .filter(|x| !feat.contains(x.clone()))
        .collect::<String>()
}

fn hash(s: &str) -> i64 {
    let mut sum = 0;
    for ch in s.chars() {
        sum += 1 << (ch as u8 - 'a' as u8);
    }
    sum
}

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let breakdown = inp
        .into_iter()
        .map(|line| line.split_once("|").expect("error"));
    let mut result = vec![];
    for data in breakdown {
        let dict: Vec<&str> = data.0.split_whitespace().collect();
        let encoded = data.1.split_whitespace();

        let mut core = vec![""; 10];
        let mut rest = vec![];
        for e in dict {
            match e.len() {
                2 => core[1] = e,
                3 => core[7] = e,
                4 => core[4] = e,
                7 => core[8] = e,
                _ => rest.push(e),
            }
        }
        let feat_cf = core[1];
        let feat_bd: &str = &*rem_feat(core[4], core[1]);
        for e in rest {
            if e.len() == 5 {
                if has_feat(e, feat_cf) {
                    core[3] = e;
                } else if has_feat(e, feat_bd) {
                    core[5] = e;
                } else {
                    core[2] = e;
                }
            } else if e.len() == 6 {
                // println!("{:?} {} {}", e, feat_cf, feat_bd);
                if has_feat(e, feat_cf) && has_feat(e, feat_bd) {
                    core[9] = e;
                } else if has_feat(e, &*feat_bd) {
                    core[6] = e;
                } else {
                    core[0] = e;
                }
            }
        }
        let mut count = 0;
        for s in core.iter() {
            count += (s.len() == 0) as i64;
        }
        if count != 0 {
            println!("{:?}", core);
            panic!("Missing sample");
        }

        let get = encoded
            .map(|s| {
                let mut idx = 0;
                let mut ans = 0;
                for c in &core {
                    if hash(s) == hash(*c) {
                        ans = idx;
                    }
                    idx += 1;
                }
                ans
            })
            .fold(0, |acc, e| acc * 10 + e);
        // println!("{:?}", get);
        result.push(get);
    }
    Ok(result.into_iter().sum())
}
