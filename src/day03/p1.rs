pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut val = vec![0i64; 64];
    let mut len = 0;
    for raw_num in inp {
        if len == 0 {
            len = raw_num.len();
        }
        let mut num = raw_num.parse::<i64>()?;
        for i in 0..len {
            let x = num % 10;
            val[i] += (x) - (1 - x);
            num /= 10;
        }
    }
    let mut gamma = 0i64;
    for i in val.into_iter().take(len).rev() {
        println!("{}", gamma);
        gamma = (gamma << 1) | (i > 0) as i64;
    }
    let rev = -1 & ((1 << len) - 1);
    Ok(gamma * (rev ^ gamma))
}
