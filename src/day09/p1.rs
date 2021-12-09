pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let depth_map: Vec<Vec<i64>> = inp
        .into_iter()
        .map(|row: &str| row.chars().map(|x| x as i64 - '0' as i64).collect())
        .collect();
    let mut result = vec![];
    // println!("{:?}", depth_map);
    let ti = depth_map.len();
    for i in 0..ti {
        let tj = depth_map[i].len();
        for j in 0..tj {
            let check_total = 4 - (i == 0 || i + 1 == ti) as i64 - (j == 0 || j + 1 == tj) as i64;
            let mut check_count = 0;
            if i > 0 {
                check_count += (depth_map[i - 1][j] > depth_map[i][j]) as i64;
            }
            if j > 0 {
                check_count += (depth_map[i][j - 1] > depth_map[i][j]) as i64;
            }
            if i + 1 < ti {
                check_count += (depth_map[i + 1][j] > depth_map[i][j]) as i64;
            }
            if j + 1 < tj {
                check_count += (depth_map[i][j + 1] > depth_map[i][j]) as i64;
            }
            if check_count == check_total {
                result.push(depth_map[i][j]);
            }
        }
    }
    Ok(result.into_iter().map(|x| x + 1).sum())
}
