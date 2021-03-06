pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let position: Vec<(usize, i64)> = inp[0]
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .fold(vec![0; 2000], |mut acc, e| {
            // Turn into Pivot table
            // so we can reduce memory expansion
            acc[e as usize] += 1;
            acc
        })
        .into_iter()
        .enumerate()
        .filter(|x| x.1 != 0)
        .collect();
    // println!("{:?}", position);
    let mut collector = vec![0; 2000];
    for i in 0..collector.len() {
        for item in &position {
            collector[i] += item.1 * i64::abs(item.0 as i64 - i as i64);
        }
    }
    println!("{:?}", collector);
    let global_min = collector.into_iter().min().unwrap();
    Ok(global_min)
}
