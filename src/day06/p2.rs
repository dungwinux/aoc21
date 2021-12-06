pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut fishtank: Vec<u64> =
        inp[0]
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .fold(vec![0; 10], |mut acc, e| {
                // Turn into Pivot table
                // so we can reduce memory expansion
                acc[e as usize] += 1;
                acc
            });
    println!("{:?}", fishtank);
    let cycle = 256;
    for _ in 0..cycle {
        fishtank[7] += fishtank[0];
        fishtank[9] += fishtank[0];
        for i in 0..9 {
            fishtank[i] = fishtank[i + 1];
        }
        fishtank[9] = 0;
        // println!("{:?}", fishtank);
    }
    Ok(fishtank.into_iter().sum::<u64>() as i64)
}
