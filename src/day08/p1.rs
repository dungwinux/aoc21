pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let breakdown = inp.into_iter().map(|line| line.split_once("|"));
    let encoded: Vec<i64> = breakdown
        .map(|x| {
            x.expect("error")
                .1
                .split_whitespace()
                .map(|s| s.len() as i64)
                .filter(|x| match x {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
        })
        .flatten()
        .collect();
    Ok(encoded.len() as i64)
}
