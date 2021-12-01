pub fn solve(inp: Vec<&str>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut prev = -1;
    let mut counter = 0;
    for raw_num in inp {
        let num = raw_num.parse::<i32>()?;
        if prev != -1 && prev < num {
            counter += 1;
        }
        prev = num;
    }
    Ok(counter)
}
