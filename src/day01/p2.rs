pub fn solve(inp: Vec<&str>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut prev = -1;
    let mut counter = 0;
    let mut v = std::collections::VecDeque::new();
    let mut sum: i32 = 0;
    for raw_num in inp {
        let num = raw_num.parse::<i32>()?;
        v.push_back(num);
        sum += num;
        if v.len() == 3 {
            println!("{}", sum);
            if prev != -1 && prev < sum {
                counter += 1;
            }
            prev = sum;
            sum -= v.pop_front().expect("You hacked the queue");
        }
    }
    Ok(counter)
}
