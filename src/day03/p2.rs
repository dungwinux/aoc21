pub fn proc(mut val_list: Vec<i64>, len: usize, ascd: bool) -> i64 {
    let mut base = 0;
    for i in 0..len {
        base |= 1 << (len - i - 1);
        println!("{:?} {}", val_list, base);
        if val_list.len() < 2 {
            break;
        }
        let loc = val_list
            .iter()
            .position(|x| x >= &base)
            .expect("Data corrupt");
        // XNOR
        if !((2 * loc <= val_list.len()) ^ ascd) {
            println!("[{:?}]", 1);
            val_list = val_list.split_off(loc);
        } else {
            println!("[{:?}]", 0);
            let off = 1 << (len - i - 1);
            base &= -(off + 1);
            val_list.truncate(loc);
        }
    }
    val_list[0]
}

pub fn solve(inp: Vec<&str>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut val_list = vec![];
    let mut len = 0;
    for raw_num in inp {
        if len == 0 {
            len = raw_num.len();
        }
        let mut num = raw_num.parse::<i64>()?;
        let mut true_val = 0;
        for i in 0..len {
            let x = num % 10;
            true_val |= x << i;
            num /= 10;
        }
        val_list.push(true_val);
    }
    val_list.sort();

    let gamma = proc(val_list.clone(), len, true);
    let epsilon = proc(val_list, len, false);
    Ok(gamma * epsilon)
}
