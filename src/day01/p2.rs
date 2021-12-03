pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let int_vec = inp
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut counter = 0;
    let mut idx = 0;
    for num in int_vec.iter() {
        if idx >= 3 && int_vec[idx - 3] < *num {
            counter += 1;
        }
        idx += 1;
    }
    Ok(counter)
}
