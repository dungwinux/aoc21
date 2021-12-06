pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut fishtank: Vec<i64> = inp[0]
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    // Naive loop
    let mut cycle = 80;
    while cycle > 0 {
        cycle -= 1;
        let mut newborn = 0;
        fishtank = fishtank
            .into_iter()
            .map(|x| {
                if x == 0 {
                    newborn += 1;
                    6
                } else {
                    x - 1
                }
            })
            .collect();
        for _ in 0..newborn {
            fishtank.push(8);
        }
    }
    Ok(fishtank.len() as i64)
}
