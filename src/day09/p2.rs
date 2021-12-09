fn move_sim(given_map: &Vec<Vec<i64>>, cover_map: &mut Vec<Vec<bool>>, x: usize, y: usize) -> i64 {
    // println!("[LOG] {} {}", x, y);
    if cover_map[x][y] {
        return 0;
    }
    let mut check_count = 1;
    cover_map[x][y] = true;
    if x > 0 && given_map[x - 1][y] > given_map[x][y] && given_map[x - 1][y] != 9 {
        check_count += move_sim(given_map, cover_map, x - 1, y);
    }
    if y > 0 && given_map[x][y - 1] > given_map[x][y] && given_map[x][y - 1] != 9 {
        check_count += move_sim(given_map, cover_map, x, y - 1);
    }
    if x + 1 < given_map.len() && given_map[x + 1][y] > given_map[x][y] && given_map[x + 1][y] != 9
    {
        check_count += move_sim(given_map, cover_map, x + 1, y);
    }
    if y + 1 < given_map[x].len()
        && given_map[x][y + 1] > given_map[x][y]
        && given_map[x][y + 1] != 9
    {
        check_count += move_sim(given_map, cover_map, x, y + 1);
    }
    check_count
}

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let depth_map: Vec<Vec<i64>> = inp
        .into_iter()
        .map(|row: &str| row.chars().map(|x| x as i64 - '0' as i64).collect())
        .collect();
    let mut low_points = vec![];
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
                low_points.push((i, j));
            }
        }
    }
    // This is intended to be put inside map HOF. However,
    // ... the basin area are not collided, thus we can just use the same cover_map
    let mut cover_map = vec![vec![false; depth_map[0].len()]; depth_map.len()];
    let mut result: Vec<i64> = low_points
        .into_iter()
        .map(|(x, y)| move_sim(&depth_map, &mut cover_map, x, y))
        .collect();
    println!("{:?}", result);
    result.sort();
    Ok(result.into_iter().rev().take(3).fold(1, |acc, e| acc * e))
}
